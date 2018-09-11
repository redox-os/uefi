//! This protocol is used to handle input and output of
//! text-based information intended for the system user during the operation of code in the boot
//! services environment. Also included here are the definitions of three console devices: one for input
//! and one each for normal output and errors.

use crate::{status::Status, Event};

/// Keystroke information for the key that was pressed.
#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct TextInputKey {
    /// If there is a pending keystroke, then ScanCode is the EFI scan code defined in
    /// Table 104.
    pub ScanCode: u16,
    /// The UnicodeChar is the actual printable
    /// character or is zero if the key does not represent a printable
    /// character (control key, function key, etc.).
    pub UnicodeChar: u16,
}

/// This protocol is used to obtain input from the ConsoleIn device. The EFI specification requires that
/// the EFI_SIMPLE_TEXT_INPUT_PROTOCOL supports the same languages as the corresponding
/// EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL.
#[repr(C)]
pub struct TextInput {
    /// Reset the ConsoleIn device.
    pub Reset: extern "win64" fn(&TextInput, bool) -> Status,
    /// Returns the next input character.
    pub ReadKeyStroke: extern "win64" fn(&TextInput, &mut TextInputKey) -> Status,
    /// Event to use with EFI_BOOT_SERVICES.WaitForEvent() to wait for a key to be available.
    pub WaitForKey: Event,
}

/// The following data values in the SIMPLE_TEXT_OUTPUT_MODE interface are read-only and are
/// changed by using the appropriate interface functions.
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct TextOutputMode {
    /// The number of modes supported by QueryMode() and SetMode().
    pub MaxMode: i32,
    /// The text mode of the output device(s).
    pub Mode: i32,
    /// The current character output attribute.
    pub Attribute: i32,
    /// The cursor’s column.
    pub CursorColumn: i32,
    /// The cursor’s row.
    pub CursorRow: i32,
    /// The cursor is currently visible or not.
    pub CursorVisible: bool,
}

/// This protocol is used to control text-based output devices.
#[repr(C)]
pub struct TextOutput {
    /// Reset the ConsoleOut device.
    pub Reset: extern "win64" fn(&TextOutput, bool) -> Status,
    /// Displays the string on the device at the current cursor location.
    pub OutputString: extern "win64" fn(&TextOutput, *const u16) -> Status,
    /// Tests to see if the ConsoleOut device supports this string.
    pub TestString: extern "win64" fn(&TextOutput, *const u16) -> Status,
    /// Queries information concerning the output device’s supported text mode.
    pub QueryMode: extern "win64" fn(&TextOutput, usize, &mut usize, &mut usize) -> Status,
    /// Sets the current mode of the output device.
    pub SetMode: extern "win64" fn(&TextOutput, usize) -> Status,
    /// Sets the foreground and background color of the text that is output.
    pub SetAttribute: extern "win64" fn(&TextOutput, usize) -> Status,
    /// Clears the screen with the currently set background color.
    pub ClearScreen: extern "win64" fn(&TextOutput) -> Status,
    /// Sets the current cursor position.
    pub SetCursorPosition: extern "win64" fn(&TextOutput, usize, usize) -> Status,
    /// Turns the visibility of the cursor on/off.
    pub EnableCursor: extern "win64" fn(&TextOutput, bool) -> Status,
    /// Reference to SIMPLE_TEXT_OUTPUT_MODE data.
    pub Mode: &'static TextOutputMode,
}
