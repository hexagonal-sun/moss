use crate::drivers::timer::uptime;
use crate::memory::uaccess::{UserCopyable, copy_to_user};
use crate::{memory::PAGE_ALLOC, process::TASK_LIST};
use core::mem::size_of;
use libkernel::memory::PAGE_SIZE;
use libkernel::{error::Result, memory::address::TUA};

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SysInfo {
    /// Seconds since boot
    pub uptime: u64,
    /// 1, 5, and 15 minute load averages
    pub loads: [u64; 3],
    /// Total usable main memory size
    pub total_ram: u64,
    /// Available memory size
    pub free_ram: u64,
    /// Amount of shared memory
    pub shared_ram: u64,
    /// Memory used by buffers
    pub buffer_ram: u64,
    /// Total swap space size
    pub total_swap: u64,
    /// Swap space still available
    pub free_swap: u64,
    /// Number of current processes
    pub procs: u32,
    /// Total high memory size
    pub total_high: u64,
    /// Available high memory size
    pub free_high: u64,
    /// Memory unit size in bytes
    pub mem_unit: u32,
}

impl SysInfo {
    pub fn new() -> SysInfo {
        // Gather memory statistics from the global page allocator.
        let page_alloc = PAGE_ALLOC.get().expect("PAGE_ALLOC must be initialised");

        let total_pages = page_alloc.total_pages();
        let free_pages = page_alloc.free_pages();

        let total_ram = (total_pages * PAGE_SIZE) as u64;
        let free_ram = (free_pages * PAGE_SIZE) as u64;

        // Count the number of processes currently known to the scheduler.
        let procs = TASK_LIST.lock_save_irq().len() as u32;

        SysInfo {
            uptime: uptime().as_secs(),
            loads: [0, 0, 0], // TODO: implement actual load averages
            total_ram,
            free_ram,
            shared_ram: 0,
            buffer_ram: 0,
            total_swap: 0,
            free_swap: 0,
            procs,
            total_high: 0,
            free_high: 0,
            mem_unit: 1, // All memory figures are given in bytes
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PaddedSysInfo {
    pub info: SysInfo,
    /// Padding to 64 bytes
    _f: [u8; 20 - (2 * size_of::<u64>()) - size_of::<u32>()],
}

impl From<SysInfo> for PaddedSysInfo {
    fn from(info: SysInfo) -> Self {
        Self {
            info,
            _f: [0; 20 - (2 * size_of::<u64>()) - size_of::<u32>()],
        }
    }
}

unsafe impl UserCopyable for PaddedSysInfo {}

pub async fn sys_sysinfo(info_ptr: TUA<PaddedSysInfo>) -> Result<usize> {
    // Build the structure in kernel memory first
    let padded = PaddedSysInfo::from(SysInfo::new());
    copy_to_user(info_ptr, padded).await?;
    Ok(0)
}
