pub use serde::{Serialize, Deserialize};
pub use vector::*;
pub use core_util::*;

pub type HashMap<K,V> = fnv::FnvHashMap<K,V>;
pub use std::collections::VecDeque;