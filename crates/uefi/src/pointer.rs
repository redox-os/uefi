use crate::prelude::*;

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct SimplePointerMode {
    pub ResolutionX: u64,
    pub ResolutionY: u64,
    pub ResolutionZ: u64,
    pub LeftButton: bool,
    pub RightButton: bool,
}

#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct SimplePointerState {
    pub RelativeMovementX: i32,
    pub RelativeMovementY: i32,
    pub RelativeMovementZ: i32,
    pub LeftButton: bool,
    pub RightButton: bool,
}

#[repr(C)]
pub struct SimplePointer {
    pub Reset: extern "efiapi" fn(&mut SimplePointer, ExtendedVerification: bool) -> Status,
    pub GetState: extern "efiapi" fn(&mut SimplePointer, State: &mut SimplePointerState) -> Status,
    pub WaitForInput: Event,
    pub Mode: &'static mut SimplePointerMode,
}

impl SimplePointer {
    pub const GUID: Guid = guid!("31878c87-0b75-11d5-9a4f-0090273fc14d");
}
