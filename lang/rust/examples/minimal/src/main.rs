#![no_std]
#![no_main]

use core::{mem::MaybeUninit, panic::PanicInfo};

use embedded_alloc::Heap;
use litex_openfpga::*;
use riscv_rt::entry;

#[global_allocator]
static HEAP: Heap = Heap::empty();

const HEAP_SIZE: usize = 200 * 1024;
static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("Panic:");
    println!("{info}");

    loop {}
}

#[entry]
fn main() -> ! {
    unsafe { HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE) };

    println!("Rust: Hello, world!");

    loop {}
}
