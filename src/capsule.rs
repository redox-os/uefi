use crate::{guid::Guid, memory::PhysicalAddress};

pub const CAPSULE_FLAGS_PERSIST_ACROSS_RESET: u32 = 0x00010000;
pub const CAPSULE_FLAGS_POPULATE_SYSTEM_TABLE: u32 = 0x00020000;
pub const CAPSULE_FLAGS_INITIATE_RESET: u32 = 0x00040000;

#[repr(C)]
pub struct CapsuleHeader {
    CapsuleGuid: Guid,
    HeaderSize: u32,
    Flags: u32,
    CapsuleImageSize: u32,
}

#[repr(C)]
pub struct CapsuleBlockDescriptor {
    pub Length: u64,
    pub DataBlock: PhysicalAddress,
}
