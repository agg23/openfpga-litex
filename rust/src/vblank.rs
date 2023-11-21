#![no_std]
#![no_main]

use ::slint::platform::software_renderer::{MinimalSoftwareWindow, RepaintBufferType};
use alloc::format;
use core::arch::asm;
use core::cell::RefCell;
use core::panic::PanicInfo;
use core::slice::{from_raw_parts, from_raw_parts_mut};
use core::sync::atomic::AtomicU32;
use core::time::Duration;
use embedded_hal::prelude::_embedded_hal_blocking_serial_Write;
use hardware::file::File;
use num_traits::float::Float;
use rmp3::{RawDecoder, Sample, MAX_SAMPLES_PER_FRAME};
use slint::platform::software_renderer::Rgb565Pixel;

extern crate alloc;

use alloc::{boxed::Box, rc::Rc};
use embedded_alloc::Heap;
use litex_hal as hal;
use litex_pac as pac;
use riscv_rt::entry;

mod hardware;
mod ui;

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

use core::mem::MaybeUninit;

use crate::ui::SlintPlatform;
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
        let frame_count = peripherals.APF_VIDEO.frame_counter.read().bits();

        let vsync = peripherals.APF_VIDEO.vsync_status.read().bits();

        if vsync != 0 {
            println!("Vsync at {counter} {frame_count}");
        }

        let vblank = peripherals.APF_VIDEO.vblank_status.read().bits();

        if vblank != 0 && !in_vblank {
            in_vblank = true;

            println!("Entered vblank at {counter} {frame_count}");
        } else if vblank == 0 && in_vblank {
            in_vblank = false;

            println!("Exited vblank at {counter} {frame_count}");
        }

        counter += 1;
    }
}
