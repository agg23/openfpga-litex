#![no_std]
#![no_main]

use ::slint::platform::software_renderer::{MinimalSoftwareWindow, RepaintBufferType};
use alloc::format;
use core::panic::PanicInfo;
use core::slice::from_raw_parts_mut;
use pac::constants;
use slint::platform::software_renderer::Rgb565Pixel;

extern crate alloc;

use alloc::boxed::Box;
use embedded_alloc::Heap;
use litex_openfpga::*;
use litex_pac as pac;
use riscv_rt::entry;

slint::include_modules!();

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

const FRAMEBUFFER_ADDRESS: *mut Rgb565Pixel = constants::VIDEO_FRAMEBUFFER_BASE as *mut Rgb565Pixel;

// This is the entry point for the application.
// It is not allowed to return.
#[entry]
fn main() -> ! {
    let peripherals = unsafe { pac::Peripherals::steal() };

    // Initialize the allocator BEFORE you use it
    unsafe { HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE) };

    println!("Rendering");

    let buffer = unsafe {
        from_raw_parts_mut(
            FRAMEBUFFER_ADDRESS,
            (constants::MAX_DISPLAY_WIDTH * constants::MAX_DISPLAY_HEIGHT) as usize,
        )
    };

    let window = MinimalSoftwareWindow::new(RepaintBufferType::NewBuffer);
    slint::platform::set_platform(Box::new(SlintPlatform::new(
        window.clone(),
        constants::CONFIG_CLOCK_FREQUENCY,
    )))
    .unwrap();

    println!("Creating UI");

    // Setup the UI.
    let ui = RTC_IDUI::new().unwrap();

    ui.show().unwrap();

    window.set_size(slint::PhysicalSize::new(
        constants::MAX_DISPLAY_WIDTH,
        constants::MAX_DISPLAY_HEIGHT,
    ));

    loop {
        slint::platform::update_timers_and_animations();

        window.draw_if_needed(|renderer| {
            renderer.render(buffer, constants::MAX_DISPLAY_WIDTH as usize);
        });

        let id_low = peripherals.APF_ID.id0.read().bits();
        let id_high = peripherals.APF_ID.id1.read().bits();

        let id = (id_high as u64) << 32 | (id_low as u64);
        let time = peripherals.APF_RTC.unix_seconds.read().bits();

        let data = ui.global::<Data>();
        data.set_id(format!("{id:x}").into());
        data.set_rtc(format!("{time}").into());
    }
}
