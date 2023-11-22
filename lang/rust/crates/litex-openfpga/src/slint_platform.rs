use alloc::rc::Rc;
use num_traits::float::Float;
use slint::platform::{software_renderer::MinimalSoftwareWindow, Platform};

pub struct SlintPlatform {
    pub window: Rc<MinimalSoftwareWindow>,
    clock_cycle_period_nanos: f64,
}

fn combine_u32(low: u32, high: u32) -> u64 {
    ((high as u64) << 32) | (low as u64)
}

impl SlintPlatform {
    pub fn new(window: Rc<MinimalSoftwareWindow>, clock_speed_hz: u32) -> Self {
        SlintPlatform {
            window,
            clock_cycle_period_nanos: 1_000_000_000.0 / (clock_speed_hz as f64),
        }
    }
}

impl Platform for SlintPlatform {
    fn create_window_adapter(
        &self,
    ) -> Result<Rc<dyn slint::platform::WindowAdapter>, slint::PlatformError> {
        // Since on MCUs, there can be only one window, just return a clone of self.window.
        // We'll also use the same window in the event loop.
        Ok(self.window.clone())
    }

    fn duration_since_start(&self) -> core::time::Duration {
        let peripherals = unsafe { litex_pac::Peripherals::steal() };

        unsafe {
            // Grab cycle count
            peripherals.TIMER0.uptime_latch.write(|w| w.bits(1));
        };

        let low_bits = peripherals.TIMER0.uptime_cycles0.read().bits();
        let high_bits = peripherals.TIMER0.uptime_cycles1.read().bits();
        let uptime_cycles = combine_u32(low_bits, high_bits);

        let duration = (self.clock_cycle_period_nanos * (uptime_cycles as f64)).floor() as u64;

        core::time::Duration::from_nanos(duration)
    }

    // optional: You can put the event loop there, or in the main function, see later
    fn run_event_loop(&self) -> Result<(), slint::PlatformError> {
        todo!();
    }
}
