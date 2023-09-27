use core::slice::from_raw_parts_mut;

use crate::println;

const FRAME_ADDRESS: *mut u32 = 0x0010_0000 as *mut u32;
const FRAMEBUFFER_FLIP_ADDRESS: *mut bool = 0x8000_1000 as *mut bool;
const VBLANK_STATUS_ADDRESS: *mut VBlankStatus = 0x8000_1004 as *mut VBlankStatus;

#[repr(C)]
pub struct VBlankStatus {
    pub is_vblank: bool,
    pub vblank_count: u8,
    _placeholder: u16,
}

pub struct Hardware;

impl Hardware {
    pub fn write_pixel(address: usize, pixel: u16) {
        *&mut Hardware::pixels()[address] = pixel as u32;
    }

    pub fn flip_framebuffers() {
        unsafe { FRAMEBUFFER_FLIP_ADDRESS.write_volatile(true) };
    }

    pub fn vblank_status() -> VBlankStatus {
        unsafe { VBLANK_STATUS_ADDRESS.read_volatile() }
    }

    fn pixels() -> &'static mut [u32] {
        unsafe { from_raw_parts_mut(FRAME_ADDRESS, 267 * 240) }
    }
}
