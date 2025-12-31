//! Timekeeping.
use web_sys::window;

/// High-resolution time in seconds.
pub fn now_secs() -> f64 {
    window().unwrap().performance().unwrap().now() / 1000.0
}

pub fn now_micros() -> u64 {
    (window().unwrap().performance().unwrap().now() * 1000.0) as u64
}
