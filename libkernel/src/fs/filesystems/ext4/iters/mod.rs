pub mod file_blocks;

use alloc::boxed::Box;
use alloc::sync::Weak;
use alloc::vec;
use alloc::vec::Vec;
use async_trait::async_trait;
use crate::error::KernelError;
use crate::fs::{DirStream, Dirent};
use crate::fs::filesystems::ext4::{Ext4Filesystem, ExtInode, BLOCK_SIZE};
use crate::fs::filesystems::ext4::inode::Ext4InodeFlags;

pub struct Ext4DirStream {
    fs: Weak<Ext4Filesystem>,
    dir: ExtInode,
    block_index: Option<u64>,
    is_first_block: bool,
    block: Vec<u8>,
    offset_within_block: usize,
    is_done: bool,
    has_htree: bool,
}

impl Ext4DirStream {
    pub fn new(fs: Weak<Ext4Filesystem>, dir: ExtInode) -> Self {
        Self {
            block: vec![0; fs.upgrade().unwrap().superblock.log_block_size as usize],
            offset_within_block: 0,
            has_htree: dir.inode.flags.contains(Ext4InodeFlags::INDEX),
            fs,
            dir,
            block_index: None,
            is_first_block: true,
            is_done: false,
        }
    }
}

#[async_trait]
impl DirStream for Ext4DirStream {
    async fn next_entry(&mut self) -> crate::error::Result<Option<Dirent>> {
        let block_index = if let Some(block_index) = self.block_index {
            block_index
        } else {
            match self.file_blocks.next() {
                Some(Ok(block_index)) => {
                    self.block_index = Some(block_index);
                    self.offset_within_block = 0;

                    block_index
                }
                Some(Err(err)) => return Err(err),
                None => {
                    self.is_done = true;
                    return Ok(None);
                }
            }
        };
        return Err(KernelError::NotSupported);
    }
}