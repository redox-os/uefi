use crate::Event;
use crate::status::Status;

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
    pub Reset: extern "win64" fn (&mut SimplePointer, ExtendedVerification: bool) -> Status,
    pub GetState: extern "win64" fn (&mut SimplePointer, State: &mut SimplePointerState) -> Status,
    pub WaitForInput: Event,
    pub Mode: &'static mut SimplePointerMode,
}
