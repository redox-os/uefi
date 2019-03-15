use core::fmt::{self, Write};

use crate::UEFI;

pub struct Stdout;

impl Write for Stdout {
    fn write_str(&mut self, string: &str) -> Result<(), fmt::Error> {
        let uefi = unsafe { &mut *UEFI };

        for c in string.chars() {
            let _ = (uefi.ConsoleOut.OutputString)(uefi.ConsoleOut, [c as u16, 0].as_ptr());
            if c == '\n' {
                let _ = (uefi.ConsoleOut.OutputString)(uefi.ConsoleOut, ['\r' as u16, 0].as_ptr());
            }
        }

        Ok(())
    }
}

pub fn _print(args: fmt::Arguments) {
    Stdout.write_fmt(args).unwrap();
}
