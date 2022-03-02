use crate::Event;
use crate::status::Status;

#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct TextInputKey {
    pub ScanCode: u16,
    pub UnicodeChar: u16,
}

#[repr(C)]
pub struct TextInput {
    pub Reset: extern "win64" fn(&TextInput, bool) -> Status,
    pub ReadKeyStroke: extern "win64" fn(&TextInput, &mut TextInputKey) -> Status,
    pub WaitForKey: Event,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct TextOutputMode {
    pub MaxMode: i32,
    pub Mode: i32,
    pub Attribute: i32,
    pub CursorColumn: i32,
    pub CursorRow: i32,
    pub CursorVisible: bool,
}

#[repr(C)]
pub struct TextOutput {
    pub Reset: extern "win64" fn(&TextOutput, bool) -> Status,
    pub OutputString: extern "win64" fn(&TextOutput, *const u16) -> Status,
    pub TestString: extern "win64" fn(&TextOutput, *const u16) -> Status,
    pub QueryMode: extern "win64" fn(&TextOutput, usize, &mut usize, &mut usize) -> Status,
    pub SetMode: extern "win64" fn(&TextOutput, usize) -> Status,
    pub SetAttribute: extern "win64" fn(&TextOutput, usize) -> Status,
    pub ClearScreen: extern "win64" fn(&TextOutput) -> Status,
    pub SetCursorPosition: extern "win64" fn(&TextOutput, usize, usize) -> Status,
    pub EnableCursor: extern "win64" fn(&TextOutput, bool) -> Status,
    pub Mode: &'static TextOutputMode,
}
