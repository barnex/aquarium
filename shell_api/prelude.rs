pub use crate::*;

pub use core_util::*;
pub use fixed_str::*;
pub use geometry::*;
pub use num_traits::AsPrimitive as _;
pub use proc_macros::*;

pub use serde::{Deserialize, Serialize};
pub use vector::*;

pub use std::num::NonZeroU8;

pub type HashSet<T> = fnv::FnvHashSet<T>;
pub type HashMap<K, V> = fnv::FnvHashMap<K, V>;
pub type Bounds = Bounds2D<i32>;
