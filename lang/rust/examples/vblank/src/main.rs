#![no_std]
#![no_main]

use core::mem::MaybeUninit;
use core::panic::PanicInfo;

extern crate alloc;

use embedded_alloc::Heap;
use litex_hal as hal;
use litex_openfpga::*;
use litex_pac as pac;
use riscv_rt::entry;

// Definition is required for uart_printer.rs to work
hal::uart! {
    UART: pac::UART,
}

// Fix for missing main functions
#[no_mangle]
fn fminf(a: f32, b: f32) -> f32 {
    if a < b {
        a
    } else {
        b
    }
}

#[no_mangle]
fn fmaxf(a: f32, b: f32) -> f32 {
    if a > b {
        a
    } else {
        b
    }
}

const HEAP_SIZE: usize = 200 * 1024;
static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("Panic:");
    println!("{info}");

    loop {}
}

#[global_allocator]
static HEAP: Heap = Heap::empty();

// This is the entry point for the application.
// It is not allowed to return.
#[entry]
fn main() -> ! {
    let peripherals = unsafe { pac::Peripherals::steal() };

    // Initialize the allocator BEFORE you use it
    unsafe { HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE) };

    let mut counter = 0;
    let mut in_vblank = false;

    loop {
        let video_data = peripherals.APF_VIDEO.video.read();
        let vblank_triggered = video_data.vblank_triggered().bit();

        let frame_count = video_data.frame_counter().bits();

        if vblank_triggered {
            println!("Vblank started at {counter} {frame_count}");
        }

        let vblank = video_data.vblank_status().bit();

        if vblank && !in_vblank {
            in_vblank = true;

            println!("Entered vblank at {counter} {frame_count}");
        } else if !vblank && in_vblank {
            in_vblank = false;

            println!("Exited vblank at {counter} {frame_count}");
        }

        counter += 1;
    }
}
