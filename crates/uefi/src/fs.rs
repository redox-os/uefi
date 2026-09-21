// SPDX-License-Identifier: MIT

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
    pub OpenVolume: unsafe extern "efiapi" fn(This: *mut Self, Root: *mut *mut File) -> Status,
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
    pub Open: unsafe extern "efiapi" fn(
        This: *mut Self,
        NewHandle: *mut *mut Self,
        FileName: *const u16,
        OpenMode: u64,
        Attributes: u64,
    ) -> Status,
    pub Close: unsafe extern "efiapi" fn(This: *mut Self) -> Status,
    pub Delete: unsafe extern "efiapi" fn(This: *mut Self) -> Status,
    pub Read: unsafe extern "efiapi" fn(This: *mut Self, BufferSize: *mut usize, Buffer: *mut u8) -> Status,
    pub Write: unsafe extern "efiapi" fn(This: *mut Self, BufferSize: *mut usize, Buffer: *const u8) -> Status,
    pub SetPosition: unsafe extern "efiapi" fn(This: *mut Self, Position: u64) -> Status,
    pub GetPosition: unsafe extern "efiapi" fn(This: *mut Self, Position: *mut u64) -> Status,
    pub GetInfo: unsafe extern "efiapi" fn(
        This: *mut Self,
        InformationType: *const Guid,
        BufferSize: *mut usize,
        Buffer: *mut u8,
    ) -> Status,
    pub SetInfo: unsafe extern "efiapi" fn(
        This: *mut Self,
        InformationType: *const Guid,
        BufferSize: *mut usize,
        Buffer: *const u8,
    ) -> Status,
    pub Flush: unsafe extern "efiapi" fn(This: *mut Self) -> Status,
}
