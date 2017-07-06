use core::fmt;

#[repr(C)]
pub struct TextInputKey {
    pub ScanCode: u16,
    pub UnicodeChar: u16,
}

#[repr(C)]
pub struct TextInput {
    pub Reset: extern "win64" fn(&TextInput, bool) -> isize,
    pub ReadKeyStroke: extern "win64" fn(&TextInput, &mut TextInputKey) -> isize,
    pub WaitForKey: *const (),
}

#[derive(Clone, Debug)]
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
    pub Reset: extern "win64" fn(&TextInput, bool) -> isize,
    pub OutputString: extern "win64" fn(&TextOutput, *const u16) -> isize,
    pub TestString: extern "win64" fn(&TextOutput, *const u16) -> isize,
    pub QueryMode: extern "win64" fn(&TextOutput, usize, &mut usize, &mut usize) -> isize,
    pub SetMode: extern "win64" fn(&TextOutput, usize) -> isize,
    pub SetAttribute: extern "win64" fn(&TextOutput, usize) -> isize,
    pub ClearScreen: extern "win64" fn(&TextOutput) -> isize,
    pub SetCursorPosition: extern "win64" fn(&TextOutput, usize, usize) -> isize,
    pub EnableCursor: extern "win64" fn(&TextOutput, bool) -> isize,
    pub Mode: &'static TextOutputMode,
}

impl fmt::Write for TextOutput {
    fn write_str(&mut self, string: &str) -> Result<(), fmt::Error> {
        for c in string.chars() {
            (self.OutputString)(self, [c as u16, 0].as_ptr());
            if c == '\n' {
                (self.OutputString)(self, ['\r' as u16, 0].as_ptr());
            }
        }

        Ok(())
    }
}
