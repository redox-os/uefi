use crate::memory::MemoryType;
use crate::prelude::*;

#[repr(C)]
pub struct LoadedImage {
    pub Revision: u32,
    pub ParentHandle: Handle,
    pub SystemTable: &'static mut SystemTable,
    pub DeviceHandle: Handle,
    pub FilePath: usize,
    pub Reserved: usize,
    pub LoadOptionsSize: u32,
    pub LoadOptions: *const u16,
    pub ImageBase: usize,
    pub ImageSize: u64,
    pub ImageCodeType: MemoryType,
    pub ImageDataType: MemoryType,
    pub Unload: extern "efiapi" fn(ImageHandle: Handle) -> Status,
}

impl LoadedImage {
    pub const GUID: Guid = guid!("5b1b31a1-9562-11d2-8e3f-00a0c969723b");
}
