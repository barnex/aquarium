use crate::*;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

impl<T: Copy + Serialize> Serialize for Cel<T> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.get().serialize(serializer)
    }
}

impl<'de, T: Copy + Deserialize<'de>> Deserialize<'de> for Cel<T> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(T::deserialize(deserializer)?.cel())
    }
}
