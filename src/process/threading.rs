use core::ffi::c_long;
use core::mem::size_of;

use crate::memory::uaccess::copy_from_user;
use crate::sched::current_task;
use crate::sync::{OnceLock, SpinLock};
use alloc::{collections::BTreeMap, sync::Arc};
use libkernel::sync::waker_set::{WakerSet, wait_until};
use libkernel::{
    error::{KernelError, Result},
    memory::address::{TUA, VA},
};

/// A per-futex wait queue holding wakers for blocked tasks.
struct FutexWaitQueue {
    wakers: WakerSet,
}

impl FutexWaitQueue {
    fn new() -> Self {
        Self {
            wakers: WakerSet::new(),
        }
    }
}

/// Global futex table mapping a user address to its wait queue.
static FUTEX_TABLE: OnceLock<SpinLock<BTreeMap<usize, Arc<SpinLock<FutexWaitQueue>>>>> =
    OnceLock::new();

pub async fn sys_set_tid_address(_tidptr: VA) -> Result<usize> {
    let tid = current_task().tid;

    // TODO: implement threading and this system call properly. For now, we just
    // return the PID as the thread id.
    Ok(tid.value() as _)
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct RobustList {
    next: TUA<RobustList>,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct RobustListHead {
    list: RobustList,
    futex_offset: c_long,
    list_op_pending: RobustList,
}

pub async fn sys_set_robust_list(head: TUA<RobustListHead>, len: usize) -> Result<usize> {
    if core::hint::unlikely(len != size_of::<RobustListHead>()) {
        return Err(KernelError::InvalidValue);
    }

    let task = current_task();
    task.robust_list.lock_save_irq().replace(head);

    Ok(0)
}

const FUTEX_WAIT: i32 = 0;
const FUTEX_WAKE: i32 = 1;
const FUTEX_PRIVATE_FLAG: i32 = 128;

pub async fn sys_futex(
    uaddr: TUA<u32>,
    op: i32,
    val: u32,
    _timeout: VA,
    _uaddr2: TUA<u32>,
    _val3: u32,
) -> Result<usize> {
    // Strip PRIVATE flag if present
    let cmd = op & !FUTEX_PRIVATE_FLAG;

    match cmd {
        FUTEX_WAIT => {
            // Fail fast if the value has already changed
            let current: u32 = copy_from_user(uaddr).await?;
            if current != val {
                return Err(KernelError::TryAgain);
            }

            // Obtain (or create) the wait-queue for this futex word
            let table = FUTEX_TABLE.get_or_init(|| SpinLock::new(BTreeMap::new()));
            let waitq_arc = {
                let mut guard = table.lock_save_irq();
                guard
                    .entry(uaddr.value())
                    .or_insert_with(|| Arc::new(SpinLock::new(FutexWaitQueue::new())))
                    .clone()
            };

            // Park the current task until the word’s value differs or it is woken
            wait_until(
                waitq_arc.clone(),
                |state| &mut state.wakers,
                |_| {
                    let cur = unsafe { core::ptr::read_volatile(uaddr.value() as *const u32) };
                    if cur != val { Some(()) } else { None }
                },
            )
            .await;

            Ok(0)
        }

        FUTEX_WAKE => {
            let nr_wake = val as usize;
            let mut woke = 0;

            if let Some(table) = FUTEX_TABLE.get() {
                if let Some(waitq_arc) = table.lock_save_irq().get(&uaddr.value()).cloned() {
                    let mut waitq = waitq_arc.lock_save_irq();
                    for _ in 0..nr_wake {
                        waitq.wakers.wake_one();
                        woke += 1;
                    }
                }
            }

            Ok(woke)
        }

        _ => Err(KernelError::NotSupported),
    }
}
