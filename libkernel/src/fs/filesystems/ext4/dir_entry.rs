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


pub(crate) fn is_dx_dir(inode: &ExtInode) -> bool {
    let fs = inode.fs.upgrade().unwrap();
    // TODO: ((inode->i_size >> sb->s_blocksize_bits) == 1) is also a possibility
    // https://github.com/torvalds/linux/blob/ea1013c1539270e372fc99854bc6e4d94eaeff66/fs/ext4/dir.c#L51C8-L51C53
    if fs.superblock.has_feature_dir_index() && (inode.inode.flags.contains(Ext4InodeFlags::INDEX)
        || (inode.attr.size >> fs.superblock.blocksize == 1)
        || inode.inode.flags.contains(Ext4InodeFlags::INLINE_DATA)) {
        true
    } else {
        false
    }
}
