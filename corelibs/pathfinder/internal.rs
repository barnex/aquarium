pub use core_util::*;
pub use serde::{Deserialize, Serialize};
pub use vector::*;

pub type HashMap<K, V> = fnv::FnvHashMap<K, V>;
pub use itertools::Itertools as _;
pub use std::collections::VecDeque;
