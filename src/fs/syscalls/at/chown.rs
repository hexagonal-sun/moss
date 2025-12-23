use core::ffi::c_char;

use libkernel::{
    error::Result,
    fs::path::Path,
    memory::address::TUA,
    proc::ids::{Gid, Uid},
};

use crate::{
    fs::{VFS, syscalls::at::resolve_at_start_node},
    memory::uaccess::cstr::UserCStr,
    process::fd_table::Fd,
    sched::current_task,
};

pub async fn sys_fchownat(dirfd: Fd, path: TUA<c_char>, owner: Uid, group: Gid) -> Result<usize> {
    let mut buf = [0; 1024];

    let task = current_task();
    let path = Path::new(UserCStr::from_ptr(path).copy_from_user(&mut buf).await?);
    let start_node = resolve_at_start_node(dirfd, path).await?;

    let node = VFS.resolve_path(path, start_node, task).await?;
    let mut attr = node.getattr().await?;

    attr.uid = owner;
    attr.gid = group;
    node.setattr(attr).await?;

    Ok(0)
}
