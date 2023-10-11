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
use slint::platform::software_renderer::{
    PremultipliedRgbaColor, RepaintBufferType, Rgb565Pixel, TargetPixel,
};
use slint::platform::Platform;
use slint::{platform::software_renderer::MinimalSoftwareWindow, Rgb8Pixel};
use slint::{Rgba8Pixel, Timer};

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

slint::include_modules!();

struct MyPlatform {
    window: Rc<MinimalSoftwareWindow>,
    // optional: some timer device from your device's HAL crate
    timer: TIMER,
    // ... maybe more devices
}

// const TEST_ADDR: *mut u32 = (0xF0001800 + 0x0028) as *mut u32;

const CLOCK_SPEED: u32 = 51_600_000;
const CYCLE_PERIOD_NANOS: f64 = 1_000_000_000.0 / (CLOCK_SPEED as f64);

fn combine_u32(low: u32, high: u32) -> u64 {
    ((high as u64) << 32) | (low as u64)
}

impl Platform for MyPlatform {
    fn create_window_adapter(
        &self,
    ) -> Result<Rc<dyn slint::platform::WindowAdapter>, slint::PlatformError> {
        // Since on MCUs, there can be only one window, just return a clone of self.window.
        // We'll also use the same window in the event loop.
        Ok(self.window.clone())
    }
    fn duration_since_start(&self) -> core::time::Duration {
        // core::time::Duration::from_micros(self.timer.get_time())
        unsafe {
            // Grab cycle count
            self.timer.registers.uptime_latch.write(|w| w.bits(1));
        };

        let low_bits = self.timer.registers.uptime_cycles0.read().bits();
        // let low_bits = unsafe { TEST_ADDR.read_volatile() };
        // println!("{low_bits}");
        let high_bits = self.timer.registers.uptime_cycles1.read().bits();
        let uptime_cycles = combine_u32(low_bits, high_bits);

        // let prev_uptime_cycles_low =
        //     LAST_UPTIME_CYCLES_LOW.load(core::sync::atomic::Ordering::Acquire);
        // let prev_uptime_cycles_high =
        //     LAST_UPTIME_CYCLES_HIGH.load(core::sync::atomic::Ordering::Acquire);

        // let prev_uptime_cycles = combine_u32(prev_uptime_cycles_low, prev_uptime_cycles_high);

        // // Should always fit in u32
        // let cycle_duration = (uptime_cycles - prev_uptime_cycles) as u32;

        let duration = (CYCLE_PERIOD_NANOS * (uptime_cycles as f64)).floor() as u64;

        // println!("{uptime_cycles} {duration}");

        // LAST_UPTIME_CYCLES_LOW.store(low_bits, core::sync::atomic::Ordering::Release);
        // LAST_UPTIME_CYCLES_HIGH.store(high_bits, core::sync::atomic::Ordering::Release);

        core::time::Duration::from_nanos(duration)
        // core::time::Duration::from_nanos(0)
    }
    // optional: You can put the event loop there, or in the main function, see later
    fn run_event_loop(&self) -> Result<(), slint::PlatformError> {
        todo!();
    }
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

const TEST_BUFFER_INTERNAL_ADDRESS: u32 = 0x40C0_0000;

const TEST_BUFFER_ADDRESS: *mut u32 = 0x40C0_0000 as *mut u32;
const TEST_PIXEL_BUFFER_ADDRESS: *mut Rgb565Pixel = 0x40C0_0000 as *mut Rgb565Pixel;
// const TEST_PIXEL_BUFFER_ADDRESS: *mut Rgba8Pixel = 0x40C0_0000 as *mut Rgba8Pixel;

const L2_CACHE_SIZE: usize = 8192;
const MAIN_RAM_BASE: *mut u32 = 0x40000000 as *mut u32;

// Cloned from libbase system.c
fn flush_l2_cache() {
    let bank = unsafe { from_raw_parts(MAIN_RAM_BASE, L2_CACHE_SIZE / 2) };

    for i in 0..2 * L2_CACHE_SIZE / 4 {
        bank[i];
    }
}

// This is the entry point for the application.
// It is not allowed to return.
#[entry]
fn main() -> ! {
    let peripherals = unsafe { pac::Peripherals::steal() };

    // Initialize the allocator BEFORE you use it
    unsafe { HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE) }

    println!("Heap created");

    let buffer = unsafe { from_raw_parts_mut(TEST_PIXEL_BUFFER_ADDRESS, 320 * 200) };

    // Initialize a window (we'll need it later).
    let window = MinimalSoftwareWindow::new(RepaintBufferType::NewBuffer);
    slint::platform::set_platform(Box::new(MyPlatform {
        window: window.clone(),
        timer: TIMER {
            sys_clk: 0,
            registers: peripherals.TIMER0,
        },
        //...
    }))
    .unwrap();

    println!("Creating UI");

    // Setup the UI.
    let ui = MyUI::new().unwrap();

    ui.show().unwrap();

    println!("Setting window size");

    window.set_size(slint::PhysicalSize::new(320, 200));

    // for _ in 0..500 {
    //     println!("Printing");
    // }

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

    // unsafe {
    //     peripherals
    //         .VIDEO_FRAMEBUFFER
    //         .dma_enable
    //         .write(|w| w.bits(0));
    //     peripherals
    //         .VIDEO_FRAMEBUFFER_VTG
    //         .enable
    //         .write(|w| w.bits(0));
    //     // peripherals
    //     //     .VIDEO_FRAMEBUFFER
    //     //     .dma_base
    //     //     .write(|w| w.bits(TEST_PIXEL_BUFFER_ADDRESS as u32));
    //     // peripherals
    //     //     .VIDEO_FRAMEBUFFER
    //     //     .dma_offset
    //     //     .write(|w| w.bits(0));
    // };

    // println!("Starting framebuffer");

    // flush_l2_cache();

    // unsafe {
    //     peripherals
    //         .VIDEO_FRAMEBUFFER_VTG
    //         .enable
    //         .write(|w| w.bits(1));
    //     peripherals
    //         .VIDEO_FRAMEBUFFER
    //         .dma_enable
    //         .write(|w| w.bits(1));
    // };
    // let mut render_index = 0;
    // let mut white = true;

    // loop {
    //     for y in 0..200 {
    //         let row_offset = (y / 8 + render_index) % 4;

    //         // let pixel = Rgb565Pixel(if white {
    //         //     0xFFFF
    //         // } else {
    //         //     match row_offset {
    //         //         0 => 0x73b8,
    //         //         1 => 0xe0b2,
    //         //         2 => 0xf6c4,
    //         //         _ => 0x0ea5,
    //         //     }
    //         // });

    //         let pixel = if white {
    //             Rgba8Pixel {
    //                 r: 0xFF,
    //                 g: 0xFF,
    //                 b: 0xFF,
    //                 a: 0xFF,
    //             }
    //         } else {
    //             match row_offset {
    //                 0 => Rgba8Pixel {
    //                     r: 0x73,
    //                     g: 0x75,
    //                     b: 0xc5,
    //                     a: 0xFF,
    //                 },
    //                 1 => Rgba8Pixel {
    //                     r: 0xe6,
    //                     g: 0x14,
    //                     b: 0x94,
    //                     a: 0xFF,
    //                 },
    //                 2 => Rgba8Pixel {
    //                     r: 0xf7,
    //                     g: 0xdb,
    //                     b: 0x21,
    //                     a: 0xFF,
    //                 },
    //                 _ => Rgba8Pixel {
    //                     r: 0x08,
    //                     g: 0xd7,
    //                     b: 0x29,
    //                     a: 0xFF,
    //                 },
    //             }
    //         };

    //         for x in 0..320 {
    //             buffer[y * 320 + x] = pixel;

    //             // for _ in 0..3000 {
    //             //     unsafe {
    //             //         asm!("nop");
    //             //     }
    //             // }
    //         }

    //         println!("{y}");
    //     }

    //     let mut spin_count = 0;

    //     println!("Pausing for {render_index}");

    //     // for _ in 0..10000000 {
    //     //     unsafe {
    //     //         asm!("nop");
    //     //     }
    //     // }

    //     white = !white;

    //     // if render_index < 3 {
    //     //     render_index += 1;
    //     // } else {
    //     //     render_index = 0;
    //     // }
    // }

    // println!("Beginning event loop");

    // slint::platform::update_timers_and_animations();

    // window.draw_if_needed(|renderer| {
    //     renderer.render(buffer, 320);
    // });

    // println!("Rendered");

    // window.request_redraw();

    // window.draw_if_needed(|renderer| {
    //     renderer.render(buffer, 320);
    // });

    // println!("Rendered 2");

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

            println!("Timer tick {value}");

            *value = 0;
        },
    );

    loop {
        slint::platform::update_timers_and_animations();

        window.draw_if_needed(|renderer| {
            renderer.render(buffer, 320);

            *draws_since_last_tick.borrow_mut() += 1;

            window.request_redraw();
        });

        // let fps_readout = ui.global::<FPSReadout>();
        // fps_readout.set_text(format!("{loop_count}").into());

        // let ui_positioner = ui.global::<UIPositioner>();

        // ui_positioner.set_x(ui_positioner.get_x() + 1.0);
    }

    // println!("0x{buffer:p}");

    // let mut frame = false;

    // loop {
    //     for x in 0..(320 * 200) {
    //         let mut alternating_pixel = x / 4;

    //         if frame {
    //             alternating_pixel += 1;
    //         }

    //         let color = match alternating_pixel % 2 {
    //             0 => 0xFF0000,
    //             1 => 0x00FF00,
    //             _ => 0x0000FF,
    //         };

    //         buffer[x] = color;
    //     }

    //     frame = !frame;
    // }

    // println!("Wrote all pixels");

    // unsafe {
    //     peripherals
    //         .VIDEO_FRAMEBUFFER
    //         .dma_enable
    //         .write(|w| w.bits(0));
    // };

    let mut last_status: u32 = peripherals.VIDEO_FRAMEBUFFER.dma_done.read().bits();

    let mut enabled = peripherals.VIDEO_FRAMEBUFFER.dma_enable.read().bits();
    let mut base = peripherals.VIDEO_FRAMEBUFFER.dma_base.read().bits();
    let mut offset = peripherals.VIDEO_FRAMEBUFFER.dma_offset.read().bits();
    let mut loop_en = peripherals.VIDEO_FRAMEBUFFER.dma_loop.read().bits();
    let mut length = peripherals.VIDEO_FRAMEBUFFER.dma_length.read().bits();

    println!(
        "Enabled: {enabled}, Done: 0x{last_status:x}, Base: 0x{base:x}, Offset: 0x{offset:x}, Length: 0x{length:x}, Loop: {loop_en}"
    );

    // unsafe {
    //     peripherals
    //         .VIDEO_FRAMEBUFFER
    //         .dma_base
    //         .write(|w| w.bits(TEST_BUFFER_ADDRESS as u32));
    //     peripherals
    //         .VIDEO_FRAMEBUFFER
    //         .dma_offset
    //         .write(|w| w.bits(0));
    //     peripherals
    //         .VIDEO_FRAMEBUFFER
    //         .dma_length
    //         .write(|w| w.bits(100));
    //     peripherals.VIDEO_FRAMEBUFFER.dma_loop.write(|w| w.bits(0));
    //     peripherals
    //         .VIDEO_FRAMEBUFFER
    //         .dma_enable
    //         .write(|w| w.bits(1));
    // }

    println!("Started DMA");

    last_status = peripherals.VIDEO_FRAMEBUFFER.dma_done.read().bits();

    enabled = peripherals.VIDEO_FRAMEBUFFER.dma_enable.read().bits();
    base = peripherals.VIDEO_FRAMEBUFFER.dma_base.read().bits();
    offset = peripherals.VIDEO_FRAMEBUFFER.dma_offset.read().bits();
    loop_en = peripherals.VIDEO_FRAMEBUFFER.dma_loop.read().bits();
    length = peripherals.VIDEO_FRAMEBUFFER.dma_length.read().bits();

    println!(
        "Enabled: {enabled}, Done: 0x{last_status:x}, Base: 0x{base:x}, Offset: 0x{offset:x}, Length: 0x{length:x}, Loop: {loop_en}"
    );

    println!("Starting done: {last_status}");

    loop {
        let new_status = peripherals.VIDEO_FRAMEBUFFER.dma_done.read().bits();

        if new_status != last_status {
            println!("Status changed: {new_status}");
            last_status = new_status;
        }
    }
}
