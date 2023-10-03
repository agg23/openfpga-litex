#![no_std]
#![no_main]

use embedded_hal;
use embedded_hal::prelude::_embedded_hal_blocking_serial_Write;
use nb;
use riscv;

extern crate panic_halt;
use litex_hal as hal;
use litex_pac as pac;
use riscv_rt::entry;

hal::uart! {
    UART: pac::UART,
}

hal::timer! {
    TIMER: pac::TIMER0,
}

// This is the entry point for the application.
// It is not allowed to return.
#[entry]
fn main() -> ! {
    let peripherals = unsafe { pac::Peripherals::steal() };

    let mut serial = UART {
        registers: peripherals.UART,
    };

    serial.bwrite_all(b"Hello world!\n").unwrap();
    serial.bflush();

    loop {}
}
