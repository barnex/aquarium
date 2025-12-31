use crate::prelude::*;

/// Hand-rolled monotonic clock provides precise, monotonically increasing time readings
/// even when running in the browser.
#[derive(Serialize, Deserialize, Default)]
pub struct MonotonicClock {
    frame: u64,

    monotonic_micros: u64,
    prev_readout: u64,

    // 📺 Rendering FPS estimate
    dt_micros: u32,
    dt_secs_smooth: f64,
}

impl MonotonicClock {
    /// To be called every time a new frame is rendered.
    pub fn tick(&mut self, readout: u64) {
        self.frame += 1;

        if readout > self.prev_readout && readout < self.prev_readout + 10_000_000 {
            let delta = readout - self.prev_readout;
            assert!(delta > 0);
            self.monotonic_micros += delta;
            self.prev_readout = readout;
            self.dt_micros = delta as u32;
            self.dt_secs_smooth = 0.9 * self.dt_secs_smooth + 0.1 * (self.dt_micros as f64 / 1e6);
        } else {
            log::debug!("re-sync: current readout {readout}, previous {}", self.prev_readout);
            self.prev_readout = readout;
            self.monotonic_micros += 100; // 👈 0.1ms minimal increment to avoid crazyness
        }
    }

    /// Monotonically increasing microseconds since an arbitrary reference time.
    pub fn micros(&self) -> u64 {
        self.monotonic_micros
    }

    /// Smoothed time between frames.
    pub fn dt_secs_smooth(&self) -> f64 {
        self.dt_secs_smooth
    }

    /// Smoothed frames per second.
    pub fn fps(&self) -> f64 {
        1.0 / self.dt_secs_smooth
    }
}
