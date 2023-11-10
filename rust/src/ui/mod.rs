use alloc::rc::Rc;
use num_traits::float::Float;
use slint::platform::{software_renderer::MinimalSoftwareWindow, Platform};

use litex_hal as hal;
use litex_pac as pac;

hal::timer! {
    TIMER: pac::TIMER0,
}

pub struct SlintPlatform {
    pub window: Rc<MinimalSoftwareWindow>,
    // optional: some timer device from your device's HAL crate
    pub timer: TIMER,
    // ... maybe more devices
}

// const TEST_ADDR: *mut u32 = (0xF0001800 + 0x0028) as *mut u32;

const CLOCK_SPEED: u32 = 51_600_000;
const CYCLE_PERIOD_NANOS: f64 = 1_000_000_000.0 / (CLOCK_SPEED as f64);

fn combine_u32(low: u32, high: u32) -> u64 {
    ((high as u64) << 32) | (low as u64)
}

impl SlintPlatform {
    pub fn new(window: Rc<MinimalSoftwareWindow>) -> Self {
        let peripherals = unsafe { pac::Peripherals::steal() };

        SlintPlatform {
            window,
            timer: TIMER {
                sys_clk: 0,
                registers: peripherals.TIMER0,
            },
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
        unsafe {
            // Grab cycle count
            self.timer.registers.uptime_latch.write(|w| w.bits(1));
        };

        let low_bits = self.timer.registers.uptime_cycles0.read().bits();
        let high_bits = self.timer.registers.uptime_cycles1.read().bits();
        let uptime_cycles = combine_u32(low_bits, high_bits);

        let duration = (CYCLE_PERIOD_NANOS * (uptime_cycles as f64)).floor() as u64;

        core::time::Duration::from_nanos(duration)
    }
    // optional: You can put the event loop there, or in the main function, see later
    fn run_event_loop(&self) -> Result<(), slint::PlatformError> {
        todo!();
    }
}
