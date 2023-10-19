#![no_std]
#![no_main]

use alloc::format;
use core::arch::asm;
use core::cell::RefCell;
use core::panic::PanicInfo;
use core::slice::{from_raw_parts, from_raw_parts_mut};
use core::sync::atomic::AtomicU32;
use core::time::Duration;
use embedded_hal::prelude::_embedded_hal_blocking_serial_Write;
use num_traits::float::Float;

extern crate alloc;

use alloc::{boxed::Box, rc::Rc};
use embedded_alloc::Heap;
use litex_hal as hal;
use litex_pac as pac;
use riscv_rt::entry;

mod hardware;

hal::uart! {
    UART: pac::UART,
}

hal::timer! {
    TIMER: pac::TIMER0,
}

// const TEST_ADDR: *mut u32 = (0xF0001800 + 0x0028) as *mut u32;

const CLOCK_SPEED: u32 = 51_600_000;
const CYCLE_PERIOD_NANOS: f64 = 1_000_000_000.0 / (CLOCK_SPEED as f64);

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

    for i in 0..4095 {
        let high = (i / 20) % 2 == 0;

        let value = if high { 0x0FFF_0FFF } else { 0x1000_1000 };

        unsafe { peripherals.MAIN.audio_out.write(|w| w.bits(value)) };
    }

    println!("{}", peripherals.MAIN.audio_buffer_fill.read().bits());

    unsafe { peripherals.MAIN.audio_playback_en.write(|w| w.bits(1)) };

    println!("{}", peripherals.MAIN.audio_buffer_fill.read().bits());

    loop {
        let buffer_fill = peripherals.MAIN.audio_buffer_fill.read().bits();

        if buffer_fill < 500 {
            let to_fill = 4096 - buffer_fill;

            for i in 0..to_fill {
                let high = (i / 20) % 2 == 0;

                let value = if high { 0x7FFF_7FFF } else { 0x8000_8000 };

                unsafe { peripherals.MAIN.audio_out.write(|w| w.bits(value)) };
            }
        }
    }

    loop {}
}
