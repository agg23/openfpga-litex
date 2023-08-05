#![no_std]
#![no_main]

use core::{arch::global_asm, panic::PanicInfo};

use embedded_hal::serial::Write;
use serial::Serial;

mod serial;

global_asm!(include_str!("init.s"));

/// A panic handler is required in Rust, this is probably the most basic one possible
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// lazy_static! {
//     static ref SERIAL: Serial = Serial;
// }

fn print(string: &str, serial: &mut Serial) {
    for (i, char) in string.chars().enumerate() {
        serial.write(i as u8 + 48);
        serial.write(char as u8);
    }
}

/// Main program function
#[no_mangle]
extern "C" fn main() -> () {
    let mut serial = Serial;
    // Example: Create a counter peripheral with base address 0x8000_0000
    // unsafe { *(0x1_0000 as *mut u32) = 12 }
    // unsafe { *(0x8000_0000 as *mut u32) = 25 }
    print("Hello world", &mut serial);
}
