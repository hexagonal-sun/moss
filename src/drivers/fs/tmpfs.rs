use crate::{arch::ArchImpl, drivers::Driver, fs::FilesystemDriver, memory::{PageOffsetTranslator, page::PgAllocGetter}};
use alloc::{boxed::Box, sync::Arc};
use async_trait::async_trait;
use libkernel::{
    error::{KernelError, Result},
    fs::{BlockDevice, Filesystem},
};
use log::warn;

pub struct TmpFsDriver {}

impl TmpFsDriver {
    pub fn new() -> Self {
        Self {}
    }
}

impl Driver for TmpFsDriver {
    fn name(&self) -> &'static str {
        "tmpfs"
    }

    fn as_filesystem_driver(self: Arc<Self>) -> Option<Arc<dyn FilesystemDriver>> {
        Some(self)
    }
}

#[async_trait]
impl FilesystemDriver for TmpFsDriver {
    async fn construct(
        &self,
        fs_id: u64,
        device: Option<Box<dyn BlockDevice>>,
    ) -> Result<Arc<dyn Filesystem>> {
        match device {
            Some(_) => {
                warn!("Unexpected block device for tmpfs");
                Err(KernelError::InvalidValue)
            }
            None => Ok(libkernel::fs::filesystems::tmpfs::TmpFs::<
                ArchImpl,
                PgAllocGetter,
                PageOffsetTranslator,
            >::new(fs_id)),
        }
    }
}
