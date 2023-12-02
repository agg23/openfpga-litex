#![no_std]
// Surpress camel case constant errors from svd2rust generated file
#![allow(non_camel_case_types)]

pub mod constants;
mod svd;

pub use svd::generic::*;
pub use svd::*;
