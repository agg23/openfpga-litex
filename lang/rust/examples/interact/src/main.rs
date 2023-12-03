#![no_std]
#![no_main]

use core::mem::MaybeUninit;
use core::panic::PanicInfo;

extern crate alloc;

use embedded_alloc::Heap;
use litex_openfpga::*;
use litex_pac as pac;
use riscv_rt::entry;

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

    let mut interact0 = peripherals.APF_INTERACT.interact0.read().bits();
    let mut interact1 = peripherals.APF_INTERACT.interact1.read().bits();
    let mut interact2 = peripherals.APF_INTERACT.interact2.read().bits();
    let mut interact3 = peripherals.APF_INTERACT.interact3.read().bits();
    let mut interact4 = peripherals.APF_INTERACT.interact4.read().bits();
    let mut interact5 = peripherals.APF_INTERACT.interact5.read().bits();
    let mut interact6 = peripherals.APF_INTERACT.interact6.read().bits();
    let mut interact7 = peripherals.APF_INTERACT.interact7.read().bits();
    let mut interact8 = peripherals.APF_INTERACT.interact8.read().bits();
    let mut interact9 = peripherals.APF_INTERACT.interact9.read().bits();
    let mut interact10 = peripherals.APF_INTERACT.interact10.read().bits();
    let mut interact11 = peripherals.APF_INTERACT.interact11.read().bits();
    let mut interact12 = peripherals.APF_INTERACT.interact12.read().bits();
    let mut interact13 = peripherals.APF_INTERACT.interact13.read().bits();
    let mut interact14 = peripherals.APF_INTERACT.interact14.read().bits();
    let mut interact15 = peripherals.APF_INTERACT.interact15.read().bits();

    println!("Reading interacts");

    unsafe { peripherals.APF_INTERACT.interact4.write(|w| w.bits(123)) };

    let mut loop_counter = 0;

    loop {
        unsafe {
            peripherals
                .APF_INTERACT
                .interact5
                .write(|w| w.bits(loop_counter))
        };

        if peripherals.APF_INTERACT.interact_changed0.read().bits() != 0 {
            let value = peripherals.APF_INTERACT.interact0.read().bits();
            println!("Updated 0: From {interact0:08x} to {value:08x}");
            interact0 = value;
        }

        if peripherals.APF_INTERACT.interact_changed1.read().bits() != 0 {
            let value = peripherals.APF_INTERACT.interact1.read().bits();
            println!("Updated 1: From {interact1:08x} to {value:08x}");
            interact1 = value;
        }

        if peripherals.APF_INTERACT.interact_changed2.read().bits() != 0 {
            let value = peripherals.APF_INTERACT.interact2.read().bits();
            println!("Updated 2: From {interact2:08x} to {value:08x}");
            interact2 = value;
        }

        if peripherals.APF_INTERACT.interact_changed3.read().bits() != 0 {
            let value = peripherals.APF_INTERACT.interact3.read().bits();
            println!("Updated 3: From {interact3:08x} to {value:08x}");
            interact3 = value;
        }

        if peripherals.APF_INTERACT.interact_changed4.read().bits() != 0 {
            let value = peripherals.APF_INTERACT.interact4.read().bits();
            println!("Updated 4: From {interact4:08x} to {value:08x}");
            interact4 = value;
        }

        if peripherals.APF_INTERACT.interact_changed5.read().bits() != 0 {
            let value = peripherals.APF_INTERACT.interact5.read().bits();
            println!("Updated 5: From {interact5:08x} to {value:08x}");
            interact5 = value;
        }

        if peripherals.APF_INTERACT.interact_changed6.read().bits() != 0 {
            let value = peripherals.APF_INTERACT.interact6.read().bits();
            println!("Updated 6: From {interact6:08x} to {value:08x}");
            interact6 = value;
        }

        if peripherals.APF_INTERACT.interact_changed7.read().bits() != 0 {
            let value = peripherals.APF_INTERACT.interact7.read().bits();
            println!("Updated 7: From {interact7:08x} to {value:08x}");
            interact7 = value;
        }

        if peripherals.APF_INTERACT.interact_changed8.read().bits() != 0 {
            let value = peripherals.APF_INTERACT.interact8.read().bits();
            println!("Updated 8: From {interact8:08x} to {value:08x}");
            interact8 = value;
        }

        if peripherals.APF_INTERACT.interact_changed9.read().bits() != 0 {
            let value = peripherals.APF_INTERACT.interact9.read().bits();
            println!("Updated 9: From {interact9:08x} to {value:08x}");
            interact9 = value;
        }

        if peripherals.APF_INTERACT.interact_changed10.read().bits() != 0 {
            let value = peripherals.APF_INTERACT.interact10.read().bits();
            println!("Updated 10: From {interact10:08x} to {value:08x}");
            interact10 = value;
        }

        if peripherals.APF_INTERACT.interact_changed11.read().bits() != 0 {
            let value = peripherals.APF_INTERACT.interact11.read().bits();
            println!("Updated 11: From {interact11:08x} to {value:08x}");
            interact11 = value;
        }

        if peripherals.APF_INTERACT.interact_changed12.read().bits() != 0 {
            let value = peripherals.APF_INTERACT.interact12.read().bits();
            println!("Updated 12: From {interact12:08x} to {value:08x}");
            interact12 = value;
        }

        if peripherals.APF_INTERACT.interact_changed13.read().bits() != 0 {
            let value = peripherals.APF_INTERACT.interact13.read().bits();
            println!("Updated 13: From {interact13:08x} to {value:08x}");
            interact13 = value;
        }

        if peripherals.APF_INTERACT.interact_changed14.read().bits() != 0 {
            let value = peripherals.APF_INTERACT.interact14.read().bits();
            println!("Updated 14: From {interact14:08x} to {value:08x}");
            interact14 = value;
        }

        if peripherals.APF_INTERACT.interact_changed15.read().bits() != 0 {
            let value = peripherals.APF_INTERACT.interact15.read().bits();
            println!("Updated 15: From {interact15:08x} to {value:08x}");
            interact15 = value;
        }

        loop_counter += 1;
    }
}
