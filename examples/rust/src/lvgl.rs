use lvgl::{Display, DrawBuffer};

pub fn init() {
    const HOR_RES: u32 = 267;
    const VER_RES: u32 = 240;

    let buffer = DrawBuffer::<{ (HOR_RES * VER_RES) as usize }>::default();

    let display = Display::register(buffer, HOR_RES, VER_RES, |refresh| {
        sim_display.draw_iter(refresh.as_pixels()).unwrap();
        for test in refresh.as_pixels::<Rgb565>() {}
    })?;
}
