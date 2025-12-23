use libkernel::{
    error::{KernelError, Result},
    proc::ids::{Gid, Uid},
};

use crate::{process::fd_table::Fd, sched::current_task};

pub async fn sys_fchown(fd: Fd, owner: Uid, group: Gid) -> Result<usize> {
    let task = current_task();
    let file = task
        .fd_table
        .lock_save_irq()
        .get(fd)
        .ok_or(KernelError::BadFd)?;

    let inode = file.inode().ok_or(KernelError::BadFd)?;
    let mut attr = inode.getattr().await?;

    attr.uid = owner;
    attr.gid = group;
    inode.setattr(attr).await?;

    Ok(0)
}
