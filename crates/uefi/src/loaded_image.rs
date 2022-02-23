use crate::Handle;
use crate::memory::MemoryType;
use crate::status::Status;
use crate::system::SystemTable;

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
    pub Unload: extern "win64" fn(ImageHandle: Handle) -> Status,
}
