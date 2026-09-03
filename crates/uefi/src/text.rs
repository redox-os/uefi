// SPDX-License-Identifier: MIT

use crate::prelude::*;

#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct TextInputKey {
    pub ScanCode: u16,
    pub UnicodeChar: u16,
}

#[repr(C)]
pub struct TextInput {
    pub Reset: unsafe extern "efiapi" fn(&TextInput, bool) -> Status,
    pub ReadKeyStroke: unsafe extern "efiapi" fn(&TextInput, &mut TextInputKey) -> Status,
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
    pub Reset: unsafe extern "efiapi" fn(&TextOutput, bool) -> Status,
    pub OutputString: unsafe extern "efiapi" fn(&TextOutput, *const u16) -> Status,
    pub TestString: unsafe extern "efiapi" fn(&TextOutput, *const u16) -> Status,
    pub QueryMode: unsafe extern "efiapi" fn(&TextOutput, usize, &mut usize, &mut usize) -> Status,
    pub SetMode: unsafe extern "efiapi" fn(&TextOutput, usize) -> Status,
    pub SetAttribute: unsafe extern "efiapi" fn(&TextOutput, usize) -> Status,
    pub ClearScreen: unsafe extern "efiapi" fn(&TextOutput) -> Status,
    pub SetCursorPosition: unsafe extern "efiapi" fn(&TextOutput, usize, usize) -> Status,
    pub EnableCursor: unsafe extern "efiapi" fn(&TextOutput, bool) -> Status,
    pub Mode: &'static TextOutputMode,
}

impl TextOutput {
    pub const GUID: Guid = guid!("387477c2-69c7-11d2-8e39-00a0c969723b");
}
