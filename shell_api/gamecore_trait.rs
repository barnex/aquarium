use std::sync::atomic::AtomicU64;

use serde::{Serialize, de::DeserializeOwned};

use crate::*;

pub trait GameCore: Default + DeserializeOwned + Serialize {
    fn tick(&mut self, now_secs: f64, events: impl Iterator<Item = InputEvent>, out: &mut Out);

    fn tick_for_logging() -> u64;
}
