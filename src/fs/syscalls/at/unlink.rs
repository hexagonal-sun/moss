use core::ffi::c_char;

use libkernel::{error::Result, fs::path::Path, memory::address::TUA};

use crate::{
    fs::{VFS, syscalls::at::resolve_at_start_node},
    memory::uaccess::cstr::UserCStr,
    process::fd_table::Fd,
};

// As defined in linux/fcntl.h ─ enables directory removal via unlinkat.
const AT_REMOVEDIR: u32 = 0x200;

/// unlinkat(2) implementation.
///
/// The semantics are:
/// - If `flags & AT_REMOVEDIR` is set, behave like `rmdir`.
/// - Otherwise behave like `unlink`.
pub async fn sys_unlinkat(dirfd: Fd, path: TUA<c_char>, flags: u32) -> Result<usize> {
    // Copy the user-provided path into kernel memory.
    let mut buf = [0u8; 1024];
    let path = Path::new(UserCStr::from_ptr(path).copy_from_user(&mut buf).await?);

    // Determine the starting inode for path resolution.
    let start_node = resolve_at_start_node(dirfd, path).await?;

    let remove_dir = flags & AT_REMOVEDIR != 0;

    VFS.unlink(path, start_node, remove_dir).await?;

    Ok(0)
}
