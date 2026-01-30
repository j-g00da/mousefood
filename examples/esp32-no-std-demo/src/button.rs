use esp_hal::gpio::Input;
use esp_hal::time::{Duration, Instant};

pub struct Button<'d> {
    pin: Input<'d>,
    last_state: bool,
    last_event: Instant,
    debounce: Duration,
}

impl<'d> Button<'d> {
    pub fn new(pin: Input<'d>, debounce: Duration) -> Self {
        let pressed = pin.is_low();
        Self {
            pin,
            last_state: pressed,
            last_event: Instant::now(),
            debounce,
        }
    }

    pub fn was_pressed(&mut self) -> bool {
        let pressed = self.pin.is_low();
        let now = Instant::now();
        let mut triggered = false;

        if pressed && !self.last_state && now - self.last_event >= self.debounce {
            self.last_event = now;
            triggered = true;
        }

        self.last_state = pressed;
        triggered
    }
}
