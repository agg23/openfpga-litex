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
use hardware::Hardware;
use serial::Serial;

use riscv as _;

use crate::slint::create_window;

mod hardware;
// mod lvgl;
mod pixel;
mod serial;
mod slint;
mod uart_printer;

global_asm!(include_str!("init.s"));

/// A panic handler is required in Rust, this is probably the most basic one possible
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // let mut serial = Serial;

    // Hardcoded in case there is an alloc failure; this will still allow us to see something
    // serial.write('p' as u8);
    // serial.write('a' as u8);
    // serial.write('n' as u8);
    // serial.write('i' as u8);
    // serial.write('c' as u8);
    // serial.write(':' as u8);
    println!("Panic:");

    // let mut output = String::new();

    // if let Some(s) = info.payload().downcast_ref::<&str>() {
    //     print(s, &mut serial);
    // } else {
    //     print("Unknown panic occurred", &mut serial);
    // }
    println!("{info}");
    // if let Err(_) = writeln!(&mut output, "{info}") {
    //     print("Unknown panic occurred", &mut serial);
    // } else {
    //     print(&output, &mut serial);
    //     write
    // }

    // Switch to partial framebuffer
    Hardware::flip_framebuffers();

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

/// Main program function
#[no_mangle]
extern "C" fn main() -> () {
    let mut serial = Serial;

    // print("Init", &mut serial);
    println!("Init");

    // Initialize the allocator BEFORE you use it
    {
        use core::mem::MaybeUninit;
        const HEAP_SIZE: usize = 1024;
        static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];

        print("Completing init", &mut serial);

        unsafe { HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE) }
    }

    // print("Hello world", &mut serial);

    // // Example: Create a counter peripheral with base address 0x8000_0000
    // // unsafe { *(0x1_0000 as *mut u32) = 12 }
    // // unsafe { *(0x8000_0000 as *mut u32) = 25 }
    // let mut data = String::new();

    // let starting_count = read_mcycle();

    // // unsafe { *ADDRESS = starting_count };

    // let _ = write!(data, "{starting_count}\n");
    // print(&data, &mut serial);

    print("Hello foo\n\r", &mut serial);

    let mut address: usize;
    let mut frame_cycle = 0;

    let mut bar_x = 0;
    let mut forward = true;

    let mut even = true;

    println!("a");
    println!("b");
    println!("c");

    create_window();

    // let mut check_x = 0;

    // loop {
    //     address = 0;

    //     for y in 0..240 {
    //         for x in 0..267 {
    //             // let color = if x >= bar_x && x < bar_x + 10 {
    //             //     0xF800
    //             // } else {
    //             //     0x07FF
    //             // };

    //             // let color = if x >= 100 && x < 150 && y >= 200 && y < 250 {
    //             //     0xF800
    //             // } else {
    //             //     0x07FF
    //             // };

    //             let color = match x % 4 {
    //                 0 => 0x64D9,
    //                 1 => 0xCCCC,
    //                 2 => 0xCB4C,
    //                 3 => 0x6359,
    //                 // _ => 0x64D9,
    //                 _ => 0x0000,
    //             };

    //             // let color = if x == 0 {
    //             //     0xF800
    //             // } else if x == 399 {
    //             //     0x001F
    //             // } else if y == 0 {
    //             //     0x9CC0
    //             // } else if y == 359 {
    //             //     0x04D3
    //             // } else {
    //             //     match frame_cycle {
    //             //         0 => 0x64D9,
    //             //         1 => 0xCCCC,
    //             //         2 => 0xCB4C,
    //             //         3 => 0x6359,
    //             //         _ => 0x64D9,
    //             //     }
    //             // };

    //             // if x == check_x {
    //             //     println!("X: {x}, color: {color:x}");
    //             // }

    //             Hardware::write_pixel(address + x, color);
    //         }

    //         // check_x += 1;

    //         // if check_x == 400 {
    //         //     check_x = 0;
    //         // }

    //         address += 267;
    //     }

    //     if forward && bar_x == 400 - 10 {
    //         forward = false;
    //     } else if !forward && bar_x == 0 {
    //         forward = true;
    //     }

    //     if forward {
    //         bar_x += 1;
    //     } else {
    //         bar_x -= 1;
    //     }

    //     // if even == 3 {
    //     //     even = 0;
    //     // } else {
    //     //     even += 1;
    //     // }
    //     // even = !even;

    //     let mut status = Hardware::vblank_status();

    //     while !status.is_vblank {
    //         // Wait until vblank
    //         status = Hardware::vblank_status();
    //         // println!("ti");
    //     }

    //     println!("V{}", status.vblank_count);
    //     // loop {
    //     //     if Hardware::is_vblank() {
    //     //         break;
    //     //     }
    //     // }

    //     Hardware::flip_framebuffers();
    //     // print("Flipping framebuffers\n", &mut serial);
    //     // println!("Flip");

    //     if frame_cycle == 3 {
    //         frame_cycle = 0;
    //     } else {
    //         frame_cycle += 1;
    //     }
    // }

    // let end_count = read_mcycle();

    // let mut data = String::new();
    // let _ = write!(data, "{end_count}\n");
    // print(&data, &mut serial);
}
