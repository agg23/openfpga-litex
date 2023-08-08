#![no_std]
#![no_main]

extern crate alloc;

use core::{
    arch::{asm, global_asm},
    panic::PanicInfo,
};

use alloc::string::String;
use core::fmt::Write;
use embedded_alloc::Heap;
use embedded_hal::prelude::_embedded_hal_serial_Write;
use serial::Serial;

use riscv as _;

mod serial;

global_asm!(include_str!("init.s"));

/// A panic handler is required in Rust, this is probably the most basic one possible
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    let mut serial = Serial;

    if let Some(s) = info.payload().downcast_ref::<&str>() {
        print(s, &mut serial);
    } else {
        print("Unknown panic occurred", &mut serial);
    }

    loop {}
}

#[global_allocator]
static HEAP: Heap = Heap::empty();

fn read_mcycle() -> u32 {
    let count: u32;
    unsafe {
        asm!(
            // "addi {count}, zero, 100",
            "csrr {}, mcycle",
            out(reg) count,
        )
    }
    count
}

fn print(string: &str, serial: &mut Serial) {
    for (i, char) in string.chars().enumerate() {
        serial.write(char as u8);
    }
}

const ADDRESS: *mut u32 = 0x8000_0000 as *mut u32;

/// Main program function
#[no_mangle]
extern "C" fn main() -> () {
    let mut serial = Serial;

    // Initialize the allocator BEFORE you use it
    {
        use core::mem::MaybeUninit;
        const HEAP_SIZE: usize = 1024;
        static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
        unsafe { HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE) }
    }

    // print("Hello world", &mut serial);

    // // Example: Create a counter peripheral with base address 0x8000_0000
    // // unsafe { *(0x1_0000 as *mut u32) = 12 }
    // // unsafe { *(0x8000_0000 as *mut u32) = 25 }
    let mut data = String::new();

    let starting_count = read_mcycle();

    // unsafe { *ADDRESS = starting_count };

    let _ = write!(data, "{starting_count}\n");
    print(&data, &mut serial);

    print("Hello world\n", &mut serial);

    let end_count = read_mcycle();

    let mut data = String::new();
    let _ = write!(data, "{end_count}\n");
    print(&data, &mut serial);
}
