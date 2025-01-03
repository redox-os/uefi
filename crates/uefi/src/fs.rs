use crate::prelude::*;
use crate::time::Time;

// Open modes
pub const FILE_MODE_READ: u64 = 0x0000000000000001;
pub const FILE_MODE_WRITE: u64 = 0x0000000000000002;
pub const FILE_MODE_CREATE: u64 = 0x8000000000000000;

// Attributes
pub const FILE_READ_ONLY: u64 = 0x01;
pub const FILE_HIDDEN: u64 = 0x02;
pub const FILE_SYSTEM: u64 = 0x04;
pub const FILE_RESERVED: u64 = 0x08;
pub const FILE_DIRECTORY: u64 = 0x10;
pub const FILE_ARCHIVE: u64 = 0x20;

#[repr(C)]
pub struct SimpleFileSystem {
    pub Revision: u64,
    pub OpenVolume: extern "efiapi" fn(&mut SimpleFileSystem, Root: &mut *mut File) -> Status,
}

impl SimpleFileSystem {
    pub const GUID: Guid = guid!("964e5b22-6459-11d2-8e39-00a0c969723b");
}

#[repr(C)]
pub struct FileInfo {
    pub Size: u64,
    pub FileSize: u64,
    pub PhysicalSize: u64,
    pub CreateTime: Time,
    pub LastAccessTime: Time,
    pub ModificationTime: Time,
    pub Attribute: u64,
    pub FileName: [u16; 256],
}

impl Default for FileInfo {
    fn default() -> Self {
        FileInfo {
            Size: Default::default(),
            FileSize: Default::default(),
            PhysicalSize: Default::default(),
            CreateTime: Default::default(),
            LastAccessTime: Default::default(),
            ModificationTime: Default::default(),
            Attribute: Default::default(),
            FileName: [0; 256],
        }
    }
}

impl FileInfo {
    pub const ID: Guid = guid!("09576e92-6d3f-11d2-8e39-00a0c969723b");
}

#[repr(C)]
pub struct File {
    pub Revision: u64,
    pub Open: extern "efiapi" fn(
        &mut File,
        NewHandle: &mut *mut File,
        FileName: *const u16,
        OpenMode: u64,
        Attributes: u64,
    ) -> Status,
    pub Close: extern "efiapi" fn(&mut File) -> Status,
    pub Delete: extern "efiapi" fn(&mut File) -> Status,
    pub Read: extern "efiapi" fn(&mut File, BufferSize: &mut usize, Buffer: *mut u8) -> Status,
    pub Write: extern "efiapi" fn(&mut File, BufferSize: &mut usize, Buffer: *const u8) -> Status,
    pub SetPosition: extern "efiapi" fn(&mut File, Position: u64) -> Status,
    pub GetPosition: extern "efiapi" fn(&mut File, Position: &mut u64) -> Status,
    pub GetInfo: extern "efiapi" fn(
        &mut File,
        InformationType: &Guid,
        BufferSize: &mut usize,
        Buffer: *mut u8,
    ) -> Status,
    pub SetInfo: extern "efiapi" fn(
        &mut File,
        InformationType: &Guid,
        BufferSize: &mut usize,
        Buffer: *const u8,
    ) -> Status,
    pub Flush: extern "efiapi" fn(&mut File) -> Status,
}
