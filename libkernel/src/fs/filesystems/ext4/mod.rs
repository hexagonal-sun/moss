//! EXT4 Filesystem Driver

#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use crate::proc::ids::{Gid, Uid};
use crate::{
    error::{KernelError, Result},
    fs::{
        FileType, Filesystem, Inode, InodeId,
        attr::{FileAttr, FilePermissions},
        blk::buffer::BlockBuffer,
    },
};
use alloc::{
    boxed::Box,
    sync::{Arc, Weak},
};
use core::error::Error;
use async_trait::async_trait;
use ext4_view::{Ext4, Ext4Read};
use crate::error::FsError;
use crate::fs::{DirStream, Dirent};
use crate::fs::pathbuf::PathBuf;

#[async_trait]
impl Ext4Read for BlockBuffer {
    async fn read(&mut self, start_byte: u64, dst: &mut [u8]) -> core::result::Result<(), Box<dyn Error + Send + Sync + 'static>> {
        Ok(self.read_at(start_byte, dst).await?)
    }
}

/// An EXT4 filesystem instance.
///
/// For now this struct only stores the underlying block buffer and an ID
/// assigned by the VFS when the filesystem is mounted.
pub struct Ext4Filesystem {
    inner: Ext4,
    id: u64,
}

impl Ext4Filesystem {
    /// Construct a new EXT4 filesystem instance.
    pub async fn new(dev: BlockBuffer, id: u64) -> Result<Arc<Self>> {
        Ok(Arc::new(Self {
            inner: Ext4::load(Box::new(dev)).await.unwrap(),
            id,
        }))
    }
}

// pub struct ExtInode {
//     pub fs: Weak<Ext4Filesystem>,
//     /// Actual on-disk inode id.
//     pub id: u64,
//     pub inode: Ext4Inode,
//     pub attr: FileAttr
// }
//
// impl ExtInode {
//     fn new(fs: Weak<Ext4Filesystem>, inode: Ext4Inode, id: InodeId) -> Self {
//         let size = ((inode.size_high as u64) << 32) | inode.size_lo as u64;
//
//         let mode_bits = inode.mode;
//         let file_type = if mode_bits.contains(Ext4InodeMode::IFDIR) {
//             FileType::Directory
//         } else if mode_bits.contains(Ext4InodeMode::IFREG) {
//             FileType::File
//         } else if mode_bits.contains(Ext4InodeMode::IFLNK) {
//             FileType::Symlink
//         } else if mode_bits.contains(Ext4InodeMode::IFSOCK) {
//             FileType::Socket
//         } else {
//             // TODO: handle other types
//             panic!("Unknown inode file type");
//         };
//         let permissions = FilePermissions::from_bits_truncate(mode_bits.bits() & 0o777);
//         let uid = Uid::new(inode.uid as u32);
//         let gid = Gid::new(inode.gid as u32);
//
//         let attr = FileAttr {
//             id,
//             file_type,
//             mode: permissions,
//             uid,
//             gid,
//             size,
//             nlinks: inode.links_count as u32,
//             ..FileAttr::default()
//         };
//         Self {
//             fs,
//             id: id.inode_id(),
//             inode,
//             attr
//         }
//     }
// }
//
// #[async_trait]
// impl Inode for ExtInode {
//     fn id(&self) -> InodeId {
//         self.attr.id
//     }
//
//     async fn read_at(&self, _offset: u64, _buf: &mut [u8]) -> Result<usize> {
//         Err(KernelError::NotSupported)
//     }
//
//     async fn write_at(&self, _offset: u64, _buf: &[u8]) -> Result<usize> {
//         Err(KernelError::NotSupported)
//     }
//
//     async fn truncate(&self, _size: u64) -> Result<()> {
//         Err(KernelError::NotSupported)
//     }
//
//     async fn getattr(&self) -> Result<FileAttr> {
//         Ok(self.attr.clone())
//     }
//
//     async fn lookup(&self, name: &str) -> Result<Arc<dyn Inode>> {
//         let fs = self.fs.upgrade().unwrap();
//         if !self.inode.is_dir() {
//             return Err(KernelError::Fs(FsError::NotADirectory));
//         }
//         if self.inode.flags.contains(Ext4InodeFlags::INDEX) {
//             // let entry = get_dir_entry_via_htree(fs, &self, name)?;
//             // return fs.read_inode(entry.inode).await.map(|inode| {
//             //     Arc::new(inode)
//             // });
//         }
//         let path = PathBuf::new();
//
//         // for entry in ReadDir::new(fs.clone(), &self, path)? {
//         //     let entry = entry?;
//         //     if entry.file_name() == name {
//         //         return fs.read_inode(entry.inode).await.map(|inode| {
//         //             Arc::new(inode)
//         //         });
//         //     }
//         // }
//
//         Err(KernelError::Fs(FsError::NotFound))
//     }
//
//     async fn create(
//         &self,
//         _name: &str,
//         _file_type: FileType,
//         _permissions: FilePermissions,
//     ) -> Result<Arc<dyn Inode>> {
//         Err(KernelError::NotSupported)
//     }
//
//     async fn unlink(&self, _name: &str) -> Result<()> {
//         Err(KernelError::NotSupported)
//     }
//
//     async fn readdir(&self, start_offset: u64) -> Result<Box<dyn DirStream>> {
//         if !self.inode.is_dir() {
//             return Err(KernelError::NotSupported);
//         }
//         let dir_stream = iters::Ext4DirStream::new(self.fs.clone(), self.clone());
//         Ok(Box::new(dir_stream))
//     }
// }
//
// #[async_trait]
// impl Filesystem for Ext4Filesystem {
//     fn id(&self) -> u64 {
//         self.id
//     }
//
//     /// Returns the root inode of the mounted EXT4 filesystem.
//     async fn root_inode(&self) -> Result<Arc<dyn Inode>> {
//         let dinode = self.read_inode(2).await?;
//         let id = InodeId::from_fsid_and_inodeid(self.id, 2);
//         Ok(Arc::new(ExtInode::new(self.this.clone(), dinode, id)))
//     }
//
//     /// Flushes any dirty data to the underlying block device.  The current
//     /// stub implementation simply forwards the request to `BlockBuffer::sync`.
//     async fn sync(&self) -> Result<()> {
//         self.dev.sync().await
//     }
// }
