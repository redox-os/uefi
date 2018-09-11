//! Can be used on any image handle to obtain information about the loaded image.

use crate::{memory::MemoryType, status::Status, system::SystemTable, Handle};

/// Each loaded image has an image handle that supports EFI_LOADED_IMAGE_PROTOCOL. When an
/// image is started, it is passed the image handle for itself. The image can use the handle to obtain its
/// relevant image data stored in the EFI_LOADED_IMAGE_PROTOCOL structure, such as its load options.
#[repr(C)]
pub struct LoadedImage {
    /// Defines the revision of the EFI_LOADED_IMAGE_PROTOCOL
    /// structure. All future revisions will be backward compatible
    /// to the current revision.
    pub Revision: u32,
    /// Parent image’s image handle. NULL if the image is loaded
    /// directly from the firmware’s boot manager.
    pub ParentHandle: Handle,
    /// The image’s EFI system table pointer.
    pub SystemTable: &'static mut SystemTable,
    /// The device handle that the EFI Image was loaded from.
    pub DeviceHandle: Handle,
    /// A pointer to the file path portion specific to DeviceHandle
    /// that the EFI Image was loaded from.
    pub FilePath: usize,
    /// Reserved. DO NOT USE.
    pub Reserved: usize,
    /// The size in bytes of LoadOptions.
    pub LoadOptionsSize: u32,
    /// A pointer to the image’s binary load options.
    pub LoadOptions: *const u16,
    /// The base address at which the image was loaded.
    pub ImageBase: usize,
    /// The size in bytes of the loaded image.
    pub ImageSize: u64,
    /// The memory type that the code sections were loaded as.
    pub ImageCodeType: MemoryType,
    /// The memory type that the data sections were loaded as.
    pub ImageDataType: MemoryType,
    /// Function that unloads the image.
    pub Unload: extern "win64" fn(ImageHandle: Handle) -> Status,
}
