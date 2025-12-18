use crate::error::KernelError;
use crate::fs::filesystems::ext4::{Ext4Filesystem, ExtInode};
use crate::fs::filesystems::ext4::inode::Ext4InodeFlags;

mod block_map;
mod extents_blocks;

enum FileBlocksInner {
    ExtentsBlocks(ExtentsBlocks),
    BlockMap(BlockMap),
}

pub(crate) struct FileBlocks(FileBlocksInner);

impl FileBlocks {
    pub(crate) fn new(fs: Ext4Filesystem, inode: &ExtInode) -> Result<Self, KernelError> {
        if inode.inode.flags.contains(Ext4InodeFlags::EXTENTS) {
            Ok(Self(FileBlocksInner::ExtentsBlocks(ExtentsBlocks::new(
                fs, inode,
            )?)))
        } else {
            Ok(Self(FileBlocksInner::BlockMap(BlockMap::new(fs, inode))))
        }
    }
}

impl Iterator for FileBlocks {
    /// Block index.
    type Item = Result<u32, KernelError>;

    fn next(&mut self) -> Option<Result<u32, KernelError>> {
        match self {
            Self(FileBlocksInner::ExtentsBlocks(iter)) => iter.next(),
            Self(FileBlocksInner::BlockMap(iter)) => iter.next(),
        }
    }
}
