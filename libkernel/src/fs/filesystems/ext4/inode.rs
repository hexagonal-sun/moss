use bitflags::bitflags;
use crate::pod::Pod;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Ext4InodeOsd1Linux {
    pub l_i_version: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Ext4InodeOsd1Hurd {
    pub h_i_translator: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Ext4InodeOsd1Masix {
    pub m_i_reserved1: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union Ext4InodeOsd1 {
    pub linux1: Ext4InodeOsd1Linux,
    pub hurd1: Ext4InodeOsd1Hurd,
    pub masix1: Ext4InodeOsd1Masix,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Ext4InodeOsd2Linux {
    pub l_i_blocks_high: u16,
    pub l_i_file_acl_high: u16,
    pub l_i_uid_high: u16,
    pub l_i_gid_high: u16,
    pub l_i_checksum_lo: u16,
    pub l_i_reserved: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Ext4InodeOsd2Hurd {
    pub h_i_reserved1: u16,
    pub h_i_mode_high: u16,
    pub h_i_uid_high: u16,
    pub h_i_gid_high: u16,
    pub h_i_author: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Ext4InodeOsd2Masix {
    pub h_i_reserved1: u16,
    pub m_i_file_acl_high: u16,
    pub m_i_reserved2: [u32; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union Ext4InodeOsd2 {
    pub linux2: Ext4InodeOsd2Linux,
    pub hurd2: Ext4InodeOsd2Hurd,
    pub masix2: Ext4InodeOsd2Masix,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Ext4Inode {
    /// File mode
    pub mode: Ext4InodeMode,
    /// Low 16 bits of Owner Uid
    pub uid: u16,
    /// Size in bytes
    pub size_lo: u32,
    /// Access time
    pub atime: u32,
    /// Inode Change time
    pub ctime: u32,
    /// Modification time
    pub mtime: u32,
    /// Deletion Time
    pub dtime: u32,
    /// Low 16 bits of Group Id
    pub gid: u16,
    /// Links count
    pub links_count: u16,
    /// Blocks count
    pub blocks_lo: u32,
    /// File flags
    pub flags: Ext4InodeFlags,
    /// OS dependent 1
    pub osd1: Ext4InodeOsd1,
    /// Pointers to blocks
    pub block: [u32; 15],
    /// File version (for NFS)
    pub generation: u32,
    /// File ACL
    pub file_acl_lo: u32,
    pub size_high: u32,
    /// Obsoleted fragment address
    pub obso_faddr: u32,
    /// OS dependent 2
    pub osd2: Ext4InodeOsd2,
    pub extra_isize: u16,
    /// crc32c(uuid+inum+inode) BE
    pub checksum_hi: u16,
    /// extra Change time      (nsec << 2 | epoch)
    pub ctime_extra: u32,
    /// extra Modification time(nsec << 2 | epoch)
    pub mtime_extra: u32,
    /// extra Access time      (nsec << 2 | epoch)
    pub atime_extra: u32,
    /// File Creation time
    pub crtime: u32,
    /// extra FileCreationtime (nsec << 2 | epoch)
    pub crtime_extra: u32,
    /// high 32 bits for 64-bit version
    pub version_hi: u32,
    /// Project ID
    pub projid: u32,
}

bitflags! {
    #[derive(Copy, Clone, Debug)]
    pub struct Ext4InodeMode: u16 {
        const IXOTH = 0x0001;
        const IWOTH = 0x0002;
        const IROTH = 0x0004;
        const IXGRP = 0x0008;
        const IWGRP = 0x0010;
        const IRGRP = 0x0020;
        const IXUSR = 0x0040;
        const IWUSR = 0x0080;
        const IRUSR = 0x0100;
        const ISVTX = 0x0200;
        const ISGID = 0x0400;
        const ISUID = 0x0800;
        const IFIFO = 0x1000;
        const IFCHR = 0x2000;
        const IFDIR = 0x4000;
        const IFBLK = 0x6000;
        const IFREG = 0x8000;
        const IFLNK = 0xA000;
        const IFSOCK = 0xC000;
    }
}

bitflags! {
    #[derive(Copy, Clone, Debug)]
    pub struct Ext4InodeFlags: u32 {
        const SECURE_DELETION = 0x00000001;
        const UNDELETE = 0x00000002;
        const COMPRESS = 0x00000004;
        const SYNC = 0x00000008;
        const IMMUTABLE = 0x00000010;
        const APPEND_ONLY = 0x00000020;
        const NODUMP = 0x00000040;
        const NOATIME = 0x00000080;
        /* Reserved for compression usage... */
        const DIRTY = 0x00000100;
        const COMPRBLK = 0x00000200;
        const NOCOMPRESS = 0x00000400;
        const ENCRYPT = 0x00000800;
        /* End compression flags -- maybe not all used */
        const INDEX = 0x00001000;
        const IMAGIC = 0x00002000;
        const JOURNAL_DATA = 0x00004000;
        const NOTAIL = 0x00008000;
        const DIRSYNC = 0x00010000;
        const TOPDIR = 0x00020000;
        const HUGE_FILE = 0x00040000;
        const EXTENTS = 0x00080000;
        const VERITY = 0x00100000;
        const EA_INODE = 0x00200000;
        const DAX = 0x02000000;
        const INLINE_DATA = 0x10000000;
        const PROJINHERIT = 0x20000000;
        const CASEFOLD = 0x40000000;
        const READONLY = 0x80000000;
    }
}

impl Ext4Inode {
    pub fn size(&self) -> u64 {
        self.size_lo as u64 | ((self.size_high as u64) << 32)
    }

    pub fn is_dir(&self) -> bool {
        self.mode.contains(Ext4InodeMode::IFDIR)
    }
}

unsafe impl Pod for Ext4InodeOsd1Linux {}
unsafe impl Pod for Ext4InodeOsd1Hurd {}
unsafe impl Pod for Ext4InodeOsd1Masix {}
unsafe impl Pod for Ext4InodeOsd1 {}
unsafe impl Pod for Ext4InodeOsd2Linux {}
unsafe impl Pod for Ext4InodeOsd2Hurd {}
unsafe impl Pod for Ext4InodeOsd2Masix {}
unsafe impl Pod for Ext4InodeOsd2 {}
unsafe impl Pod for Ext4Inode {}
