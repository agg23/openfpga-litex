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

const DISPLAY_WIDTH: usize = 267;
const DISPLAY_HEIGHT: usize = 240;

const FRAMEBUFFER_ADDRESS: *mut Rgb565Pixel = 0x40C0_0000 as *mut Rgb565Pixel;

const MP3_BANK_ADDRESS_A: *mut u8 = 0x4030_0000 as *mut u8;
const MP3_BANK_ADDRESS_B: *mut u8 = 0x4040_0000 as *mut u8;

const READ_LENGTH: usize = 0x10000;

fn combine_u32(low: u32, high: u32) -> u64 {
    ((high as u64) << 32) | (low as u64)
}

fn get_cycle_count() -> u64 {
    let peripherals = unsafe { pac::Peripherals::steal() };

    unsafe {
        // Grab cycle count
        peripherals.TIMER0.uptime_latch.write(|w| w.bits(1));
    };

    let low_bits = peripherals.TIMER0.uptime_cycles0.read().bits();
    // let low_bits = unsafe { TEST_ADDR.read_volatile() };
    // println!("{low_bits}");
    let high_bits = peripherals.TIMER0.uptime_cycles1.read().bits();
    let uptime_cycles = combine_u32(low_bits, high_bits);

    // let prev_uptime_cycles_low =
    //     LAST_UPTIME_CYCLES_LOW.load(core::sync::atomic::Ordering::Acquire);
    // let prev_uptime_cycles_high =
    //     LAST_UPTIME_CYCLES_HIGH.load(core::sync::atomic::Ordering::Acquire);

    // let prev_uptime_cycles = combine_u32(prev_uptime_cycles_low, prev_uptime_cycles_high);

    // // Should always fit in u32
    // let cycle_duration = (uptime_cycles - prev_uptime_cycles) as u32;

    (CYCLE_PERIOD_NANOS * (uptime_cycles as f64)).floor() as u64
}

fn render_init() {
    let buffer = unsafe { from_raw_parts_mut(FRAMEBUFFER_ADDRESS, DISPLAY_WIDTH * DISPLAY_HEIGHT) };

    let window = MinimalSoftwareWindow::new(RepaintBufferType::NewBuffer);
    slint::platform::set_platform(Box::new(SlintPlatform::new(window.clone()))).unwrap();

    println!("Creating UI");

    // Setup the UI.
    let ui = AudioUI::new().unwrap();

    ui.show().unwrap();

    window.set_size(slint::PhysicalSize::new(
        DISPLAY_WIDTH as u32,
        DISPLAY_HEIGHT as u32,
    ));

    slint::platform::update_timers_and_animations();

    window.draw_if_needed(|renderer| {
        renderer.render(buffer, DISPLAY_WIDTH);
    });
}

// This is the entry point for the application.
// It is not allowed to return.
#[entry]
fn main() -> ! {
    let peripherals = unsafe { pac::Peripherals::steal() };

    // Initialize the allocator BEFORE you use it
    unsafe { HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE) };

    println!("Rendering");

    render_init();

    let mut decoder = RawDecoder::new();
    let mut mp3_sample_buffer = [Sample::default(); MAX_SAMPLES_PER_FRAME];

    let file_size = File::size(0) as usize;

    let mut bank_start_offset = 0;
    let mut read_offset = 0;

    let mut current_bank_offset = 0;
    let mut inactive_bank_offset = READ_LENGTH - MAX_SAMPLES_PER_FRAME;

    println!("Starting");

    File::request_read(0, READ_LENGTH as u32, MP3_BANK_ADDRESS_A as u32, 0);

    File::block_op_complete();

    File::request_read(
        inactive_bank_offset as u32,
        READ_LENGTH as u32,
        MP3_BANK_ADDRESS_B as u32,
        0,
    );

    // We have data in the buffer ready to be used
    let read_data_a = unsafe { from_raw_parts(MP3_BANK_ADDRESS_A, READ_LENGTH) };
    let read_data_b = unsafe { from_raw_parts(MP3_BANK_ADDRESS_B, READ_LENGTH) };

    let mut use_b_bank = false;

    loop {
        let read_data = if use_b_bank { read_data_b } else { read_data_a };

        if let Some((frame, bytes_consumed)) =
            decoder.next(&read_data[read_offset..], &mut mp3_sample_buffer)
        {
            read_offset += bytes_consumed;

            if READ_LENGTH - read_offset + bank_start_offset < MAX_SAMPLES_PER_FRAME {
                // Samples includes both channels, so 1152 * 2
                // Not enough space for one more frame in this bank, so we're going to switch banks and request new data

                // Make sure previous reads are complete
                File::block_op_complete();

                // End of bank. See how far into the other bank we are
                // We will start at this new offset in the swapped bank
                bank_start_offset = (current_bank_offset + read_offset) - inactive_bank_offset;
                read_offset = bank_start_offset;

                use_b_bank = !use_b_bank;

                current_bank_offset = inactive_bank_offset;
                inactive_bank_offset =
                    current_bank_offset + read_offset + READ_LENGTH - MAX_SAMPLES_PER_FRAME;

                let storage_address = if use_b_bank {
                    // Request A bank
                    MP3_BANK_ADDRESS_A
                } else {
                    MP3_BANK_ADDRESS_B
                };

                let distance_from_end = file_size as i32 - inactive_bank_offset as i32;
                // Poor std's min
                let read_length = if distance_from_end >= READ_LENGTH as i32 {
                    READ_LENGTH as i32
                } else {
                    distance_from_end
                };

                // TODO: For some reason the very last block doesn't completely read. Cannot figure out why
                // this is happening. Hardware seems to be working correctly, but there's basically no
                // software here. Maybe Wishbone issue?

                if read_length > 0 {
                    println!("Fetching {read_length:x} for {inactive_bank_offset:x}");

                    File::request_read(
                        inactive_bank_offset as u32,
                        read_length as u32,
                        storage_address as u32,
                        0,
                    );
                }
            }

            match frame {
                rmp3::Frame::Audio(audio) => {
                    let mut buffer_fill = peripherals.MAIN.audio_buffer_fill.read().bits();

                    while buffer_fill > 10 {
                        // Busy wait until the buffer is mostly empty
                        buffer_fill = peripherals.MAIN.audio_buffer_fill.read().bits();
                    }

                    if audio.channels() != 2 {
                        println!("Unexpected channel count {}", audio.channels());
                    }

                    let two_channel_samples = unsafe {
                        from_raw_parts(audio.samples().as_ptr() as *const u32, audio.sample_count())
                    };

                    for i in 0..audio.sample_count() {
                        let value = two_channel_samples[i];
                        unsafe { peripherals.MAIN.audio_out.write(|w| w.bits(value)) };
                    }

                    unsafe { peripherals.MAIN.audio_playback_en.write(|w| w.bits(1)) };
                }
                rmp3::Frame::Other(_) => {
                    // Skip
                    println!("Other frame data");
                }
            }

            if current_bank_offset + read_offset >= file_size {
                break;
            }
        } else {
            println!("Failed to read frame");
        }
    }

    println!("Finished reading");

    loop {}
}
