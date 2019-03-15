use core::fmt::{self, Write};

pub struct Stdout;

impl Write for Stdout {
    fn write_str(&mut self, string: &str) -> Result<(), fmt::Error> {
        let st = crate::system_table();

        for c in string.chars() {
            let _ = (st.ConsoleOut.OutputString)(st.ConsoleOut, [c as u16, 0].as_ptr());
            if c == '\n' {
                let _ = (st.ConsoleOut.OutputString)(st.ConsoleOut, ['\r' as u16, 0].as_ptr());
            }
        }

        Ok(())
    }
}

pub fn _print(args: fmt::Arguments) {
    Stdout.write_fmt(args).unwrap();
}
