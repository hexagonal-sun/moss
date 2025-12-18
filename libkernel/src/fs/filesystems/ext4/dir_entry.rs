use crate::fs::filesystems::ext4::{ExtInode, EXT4_NAME_LEN};
use crate::fs::filesystems::ext4::inode::Ext4InodeFlags;
use crate::pod::Pod;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Ext4DirEntry {
    /// Inode number
    pub inode: u32,
    /// Directory entry length
    pub rec_len: u16,
    /// Name length
    pub name_len: u16,
    /// File name
    pub name: [u8; EXT4_NAME_LEN],
}

unsafe impl Pod for Ext4DirEntry {}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Ext4DirEntry2 {
    /// Inode number
    pub inode: u32,
    /// Directory entry length
    pub rec_len: u16,
    /// Name length
    pub name_len: u8,
    /// File type
    pub file_type: u8,
    /// File name
    pub name: [u8; EXT4_NAME_LEN],
}

unsafe impl Pod for Ext4DirEntry2 {}
