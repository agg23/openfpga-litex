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

slint::include_modules!();

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

const DISPLAY_WIDTH: usize = 266;
const DISPLAY_HEIGHT: usize = 240;

const FRAMEBUFFER_ADDRESS: *mut Rgb565Pixel = 0x40C0_0000 as *mut Rgb565Pixel;

// This is the entry point for the application.
// It is not allowed to return.
#[entry]
fn main() -> ! {
    let peripherals = unsafe { pac::Peripherals::steal() };

    // Initialize the allocator BEFORE you use it
    unsafe { HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE) };

    println!("Rendering");

    let buffer = unsafe { from_raw_parts_mut(FRAMEBUFFER_ADDRESS, DISPLAY_WIDTH * DISPLAY_HEIGHT) };

    let window = MinimalSoftwareWindow::new(RepaintBufferType::NewBuffer);
    slint::platform::set_platform(Box::new(SlintPlatform::new(window.clone()))).unwrap();

    println!("Creating UI");

    // Setup the UI.
    let ui = RTC_IDUI::new().unwrap();

    ui.show().unwrap();

    window.set_size(slint::PhysicalSize::new(
        DISPLAY_WIDTH as u32,
        DISPLAY_HEIGHT as u32,
    ));

    loop {
        slint::platform::update_timers_and_animations();

        window.draw_if_needed(|renderer| {
            renderer.render(buffer, DISPLAY_WIDTH);
        });

        let id_low = peripherals.APF_ID.id0.read().bits();
        let id_high = peripherals.APF_ID.id1.read().bits();

        let id = (id_high as u64) << 32 | (id_low as u64);
        let time = peripherals.APF_RTC.unix_seconds.read().bits();

        let data = ui.global::<Data>();
        data.set_id(format!("{id:x}").into());
        data.set_rtc(format!("{time}").into());
    }

    loop {}
}
