use crate::*;
use serde::{Serialize, de::DeserializeOwned};

pub trait GameCore: Default + DeserializeOwned + Serialize + 'static {
    fn tick(&mut self, unix_micros: u64, events: impl Iterator<Item = InputEvent>, out: &mut Out);

    fn reset(&mut self);
}
