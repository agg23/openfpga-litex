use alloc::{boxed::Box, rc::Rc};
use slint::{
    platform::{software_renderer::MinimalSoftwareWindow, Platform},
    PhysicalSize,
};

use crate::{hardware::Hardware, println};

struct SlintPlatform {
    window: Rc<MinimalSoftwareWindow>,
}

impl Platform for SlintPlatform {
    fn create_window_adapter(
        &self,
    ) -> Result<Rc<dyn slint::platform::WindowAdapter>, slint::PlatformError> {
        Ok(self.window.clone())
    }
}

pub fn create_window() {
    let window = MinimalSoftwareWindow::new(
        slint::platform::software_renderer::RepaintBufferType::SwappedBuffers,
    );
    slint::platform::set_platform(Box::new(SlintPlatform {
        window: window.clone(),
    }))
    .unwrap();

    window.set_size(PhysicalSize::new(267, 240));

    loop {
        slint::platform::update_timers_and_animations();

        println!("Frame");

        window.draw_if_needed(|renderer| {
            renderer.render(Hardware::slint_pixels(), 267);

            Hardware::flip_framebuffers();
        });
    }
}
