use litex_hal;
use litex_pac;

litex_hal::uart! {
    UART: litex_pac::UART,
}

#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => {{
        {
            use core::fmt::Write;
            // Hopefully this is zero cost
            let peripherals = unsafe { litex_pac::Peripherals::steal() };

            let mut serial = UART::new(peripherals.UART);

            writeln!(serial, $($arg)*).ok();
        }
    }};
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {{
        {
            use core::fmt::Write;
            // Hopefully this is zero cost
            let peripherals = unsafe { litex_pac::Peripherals::steal() };

            let mut serial = UART::new(peripherals.UART);

            write!(serial, $($arg)*).ok();
        }
    }};
}
