#![no_std]

extern crate alloc;

pub mod file;
#[cfg(feature = "slint")]
pub mod slint_platform;
pub mod uart_printer;

pub use file::*;
pub use uart_printer::*;

#[cfg(feature = "slint")]
pub use slint_platform::*;
