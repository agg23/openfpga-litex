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
use hardware::file::File;
use num_traits::float::Float;
use rmp3::{RawDecoder, Sample, MAX_SAMPLES_PER_FRAME};

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

const MP3_HEAP_ADDRESS: *mut u8 = 0x4030_0000 as *mut u8;

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

// This is the entry point for the application.
// It is not allowed to return.
#[entry]
fn main() -> ! {
    let peripherals = unsafe { pac::Peripherals::steal() };

    // Initialize the allocator BEFORE you use it
    unsafe { HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE) };

    let mut decoder = RawDecoder::new();
    let mut mp3_sample_buffer = [Sample::default(); MAX_SAMPLES_PER_FRAME];

    let file_size = File::size(0);
    let mut file_offset = 0;
    let mut started_playback = false;

    let read_length = 0x4000;
    let mut read_offset = 0;

    File::request_read(0, read_length, MP3_HEAP_ADDRESS as u32, 0);

    File::block_op_complete();

    // We have data in the buffer ready to be used
    let read_data = unsafe { from_raw_parts(MP3_HEAP_ADDRESS, read_length as usize) };

    loop {
        println!("Reading frame block");
        let start = get_cycle_count();

        if let Some((frame, bytes_consumed)) =
            decoder.next(&read_data[read_offset..], &mut mp3_sample_buffer)
        {
            let end = get_cycle_count();

            // Immediately start requesting the next batch of audio
            file_offset += bytes_consumed as u32;
            read_offset += bytes_consumed;

            println!("Consumed {bytes_consumed:x} over {} nanos", end - start);

            if read_length as usize - read_offset < 1152 {
                // Not enough space for one more frame in this sample
                // We need new data
                println!("Reading at 0x{file_offset:x}");
                File::request_read(file_offset, read_length, MP3_HEAP_ADDRESS as u32, 0);
                read_offset = 0;
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

                    println!("Wrote frame block of {} samples", audio.sample_count());

                    if !started_playback {
                        unsafe { peripherals.MAIN.audio_playback_en.write(|w| w.bits(1)) };
                        started_playback = true;
                    }
                }
                rmp3::Frame::Other(_) => {
                    // Skip
                    println!("Other frame data");
                }
            }

            if file_offset >= file_size {
                break;
            }

            // We will do nothing if we didn't start a read
            if read_offset == 0 {
                let start = get_cycle_count();
                File::block_op_complete();
                let end = get_cycle_count();

                println!("Waited for {} nanos", end - start);
            }
        } else {
            println!("Failed to read frame");
        }
    }

    // for i in 0..4095 {
    //     let high = (i / 20) % 2 == 0;

    //     let value = if high { 0x0FFF_0FFF } else { 0x1000_1000 };

    //     unsafe { peripherals.MAIN.audio_out.write(|w| w.bits(value)) };
    // }

    // // println!("{}", peripherals.MAIN.audio_buffer_fill.read().bits());

    // unsafe { peripherals.MAIN.audio_playback_en.write(|w| w.bits(1)) };

    // // println!("{}", peripherals.MAIN.audio_buffer_fill.read().bits());

    // loop {
    //     let buffer_fill = peripherals.MAIN.audio_buffer_fill.read().bits();

    //     if buffer_fill < 500 {
    //         let to_fill = 4096 - buffer_fill;

    //         for i in 0..to_fill {
    //             let high = if peripherals.MAIN.cont1_key.read().bits() & 0x10 != 0 {
    //                 // A
    //                 (i / 20) % 2 == 0
    //             } else {
    //                 (i / 10) % 2 == 0
    //             };

    //             let value = if high { 0x7FFF_7FFF } else { 0x8000_8000 };

    //             unsafe { peripherals.MAIN.audio_out.write(|w| w.bits(value)) };
    //         }
    //     }
    // }

    loop {}
}
