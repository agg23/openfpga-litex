#![no_std]
#![no_main]

extern crate alloc;

use alloc::format;
use core::cell::RefCell;
use core::panic::PanicInfo;
use core::slice::from_raw_parts_mut;
use core::time::Duration;
use pac::constants;
use slint::platform::software_renderer::MinimalSoftwareWindow;
use slint::platform::software_renderer::{RepaintBufferType, Rgb565Pixel};
use slint::Timer;

use alloc::{boxed::Box, rc::Rc};
use embedded_alloc::Heap;
use litex_hal as hal;
use litex_openfpga::*;
use litex_pac as pac;
use riscv_rt::entry;

hal::timer! {
    TIMER: pac::TIMER0,
}

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

const TEST_BUFFER_INTERNAL_ADDRESS: u32 = constants::VIDEO_FRAMEBUFFER_BASE;

const TEST_PIXEL_BUFFER_ADDRESS: *mut Rgb565Pixel =
    constants::VIDEO_FRAMEBUFFER_BASE as *mut Rgb565Pixel;

const TEST_WORD_ADDRESS: *mut u32 = 0x4030_0000 as *mut u32;

// This is the entry point for the application.
// It is not allowed to return.
#[entry]
fn main() -> ! {
    let peripherals = unsafe { pac::Peripherals::steal() };

    // Initialize the allocator BEFORE you use it
    unsafe { HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE) }

    println!("Heap created");

    let buffer = unsafe {
        from_raw_parts_mut(
            TEST_PIXEL_BUFFER_ADDRESS,
            (constants::MAX_DISPLAY_WIDTH * constants::MAX_DISPLAY_HEIGHT) as usize,
        )
    };

    // Initialize a window (we'll need it later).
    let window = MinimalSoftwareWindow::new(RepaintBufferType::NewBuffer);
    slint::platform::set_platform(Box::new(SlintPlatform::new(window.clone(), 57_120_000)))
        .unwrap();

    println!("Creating UI");

    // Setup the UI.
    let ui = MyUI::new().unwrap();

    ui.show().unwrap();

    println!("Setting window size");

    window.set_size(slint::PhysicalSize::new(
        constants::MAX_DISPLAY_WIDTH as u32,
        constants::MAX_DISPLAY_HEIGHT as u32,
    ));

    // FB Off
    unsafe {
        peripherals
            .VIDEO_FRAMEBUFFER_VTG
            .enable
            .write(|w| w.bits(0));
        peripherals
            .VIDEO_FRAMEBUFFER
            .dma_enable
            .write(|w| w.bits(0));
    }

    println!("FB Off");

    // Set read page
    unsafe {
        peripherals
            .VIDEO_FRAMEBUFFER
            .dma_base
            .write(|w| w.bits(TEST_BUFFER_INTERNAL_ADDRESS));
    }

    // FB On
    unsafe {
        peripherals
            .VIDEO_FRAMEBUFFER_VTG
            .enable
            .write(|w| w.bits(1));
        peripherals
            .VIDEO_FRAMEBUFFER
            .dma_enable
            .write(|w| w.bits(1));
    }

    let timer = Timer::default();

    let shared_ui = Rc::new(RefCell::new(ui));
    let draws_since_last_tick = Rc::new(RefCell::<u32>::new(0));

    let timer_shared_ui = shared_ui.clone();
    let draws_since_last_tick_timer = draws_since_last_tick.clone();

    timer.start(
        slint::TimerMode::Repeated,
        Duration::from_secs(1),
        move || {
            let mut value = draws_since_last_tick_timer.borrow_mut();

            let ui = timer_shared_ui.borrow();
            let fps_readout = ui.global::<FPSReadout>();
            fps_readout.set_text(format!("{value}").into());

            println!("FPS: {value}");

            // let current_value = unsafe { MAIN_RAM_BASE.read_volatile() };
            let current_value = unsafe { TEST_WORD_ADDRESS.read_volatile() };

            println!("Mem value: {current_value:x}");

            *value = 0;
        },
    );

    let mut last_address = 0;
    let mut button_pressed = false;

    loop {
        slint::platform::update_timers_and_animations();

        window.draw_if_needed(|renderer| {
            renderer.render(buffer, constants::MAX_DISPLAY_WIDTH as usize);

            let ui = shared_ui.borrow();

            let ui_positioner = ui.global::<UIPositioner>();

            let mut x = ui_positioner.get_x();
            let mut y = ui_positioner.get_y();

            let cont1_key = peripherals.APF_INPUT.cont1_key.read().bits();

            if cont1_key & 0x1 != 0 {
                // Up
                y -= 1.0;
            } else if cont1_key & 0x2 != 0 {
                // Down
                y += 1.0;
            }

            if cont1_key & 0x4 != 0 {
                // Left
                x -= 1.0;
            } else if cont1_key & 0x8 != 0 {
                // Right
                x += 1.0;
            }

            if cont1_key & 0x100 != 0 {
                if !button_pressed {
                    button_pressed = true;

                    unsafe {
                        peripherals
                            .VIDEO_FRAMEBUFFER_VTG
                            .enable
                            .write(|w| w.bits(0));
                        peripherals
                            .VIDEO_FRAMEBUFFER
                            .dma_enable
                            .write(|w| w.bits(0));

                        peripherals
                            .VIDEO_FRAMEBUFFER
                            .dma_offset
                            .write(|w| w.bits(0));
                    }

                    println!("FB Off");

                    // Set read page
                    unsafe {
                        peripherals
                            .VIDEO_FRAMEBUFFER
                            .dma_base
                            .write(|w| w.bits(TEST_BUFFER_INTERNAL_ADDRESS));
                    }

                    // FB On
                    unsafe {
                        peripherals
                            .VIDEO_FRAMEBUFFER_VTG
                            .enable
                            .write(|w| w.bits(1));
                        peripherals
                            .VIDEO_FRAMEBUFFER
                            .dma_enable
                            .write(|w| w.bits(1));
                    }

                    println!("FB On");
                }
            } else {
                button_pressed = false;
            }

            let current_address = peripherals.APF_BRIDGE.current_address.read().bits();

            if current_address != last_address {
                last_address = current_address;

                println!("Address: {current_address:x}")
            }

            let status = peripherals.APF_BRIDGE.status.read().bits();

            if status > 0 {
                println!("Finished write");
            }

            ui_positioner.set_x(x);
            ui_positioner.set_y(y);

            *draws_since_last_tick.borrow_mut() += 1;

            window.request_redraw();
        });
    }
}
