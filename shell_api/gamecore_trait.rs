use crate::*;
use serde::{Serialize, de::DeserializeOwned};

pub trait GameCore: Default + DeserializeOwned + Serialize + 'static {
    fn tick(&mut self, now_secs: f64, events: impl Iterator<Item = InputEvent>, out: &mut Out);

    fn reset(&mut self);
}
