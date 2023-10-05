#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::slice::from_raw_parts_mut;

extern crate alloc;

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

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("Panic:");
    println!("{info}");

    loop {}
}

#[global_allocator]
static HEAP: Heap = Heap::empty();

const TEST_BUFFER_ADDRESS: *mut u32 = 0x40C0_0000 as *mut u32;

// This is the entry point for the application.
// It is not allowed to return.
#[entry]
fn main() -> ! {
    let peripherals = unsafe { pac::Peripherals::steal() };

    // Initialize the allocator BEFORE you use it
    {
        use core::mem::MaybeUninit;
        const HEAP_SIZE: usize = 1024;
        static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];

        println!("Completing init");

        unsafe { HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE) }
    }

    let buffer = unsafe { from_raw_parts_mut(TEST_BUFFER_ADDRESS, 320 * 200) };

    println!("0x{buffer:p}");

    let mut frame = false;

    loop {
        for x in 0..(320 * 200) {
            let mut alternating_pixel = x / 4;

            if frame {
                alternating_pixel += 1;
            }

            let color = match alternating_pixel % 2 {
                0 => 0xFF0000,
                1 => 0x00FF00,
                _ => 0x0000FF,
            };

            buffer[x] = color;
        }

        frame = !frame;
    }

    println!("Wrote all pixels");

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
