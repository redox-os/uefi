// SPDX-License-Identifier: MIT

use crate::prelude::*;

#[repr(C)]
pub struct BlockIoMedia {
    pub MediaId: u32,
    pub RemovableMedia: bool,
    pub MediaPresent: bool,
    pub LogicalPartition: bool,
    pub ReadOnly: bool,
    pub WriteCaching: bool,
    pub BlockSize: u32,
    pub IoAlign: u32,
    pub LastBlock: u64,
}

#[repr(C)]
pub struct BlockIo {
    pub Revision: u64,
    pub Media: *const BlockIoMedia,
    pub Reset: unsafe extern "efiapi" fn(This: *mut Self, ExtendedVerification: bool) -> Status,
    pub ReadBlocks: unsafe extern "efiapi" fn(
        This: *mut Self,
        MediaId: u32,
        LBA: u64,
        BufferSize: usize,
        Buffer: *mut u8,
    ) -> Status,
    pub WriteBlocks: unsafe extern "efiapi" fn(
        This: *mut Self,
        MediaId: u32,
        LBA: u64,
        BufferSize: usize,
        Buffer: *const u8,
    ) -> Status,
    pub FlushBlocks: unsafe extern "efiapi" fn(This: *mut Self) -> Status,
}

impl BlockIo {
    pub const GUID: Guid = guid!("964e5b21-6459-11d2-8e39-00a0c969723b");
}
