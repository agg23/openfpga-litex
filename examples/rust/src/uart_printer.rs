use core::fmt::Write;

#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => {{
        {
            use core::fmt::Write;
            writeln!($crate::uart_printer::Printer, $($arg)*).ok();
        }
    }};
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {{
        {
            use core::fmt::Write;
            write!($crate::uart_printer::Printer, $($arg)*).ok();
        }
    }};
}

pub struct Printer;

const ADDRESS: *mut u8 = 0x8000_0000 as *mut u8;

impl Write for Printer {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for byte in s.as_bytes() {
            unsafe { ADDRESS.write_volatile(*byte) };
        }

        Ok(())
    }
}
