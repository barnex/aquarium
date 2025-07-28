pub use crate::*;
pub use geometry::*;
pub use num_traits::AsPrimitive;
pub use vector::*;

pub use fixed_str::*;
pub use proc_macros::*;

pub use anyhow::{Result, anyhow};
pub use itertools::Itertools as _;
pub use rand::{Rng, SeedableRng};
pub use rand_chacha::ChaCha8Rng;
pub use serde::{Deserialize, Serialize};
pub use std::collections::VecDeque;
pub use core_util::*;

pub use std::cell::Cell;
pub use std::fmt::Write as _;
pub use std::fmt::Debug;

pub type HashSet<T> = fnv::FnvHashSet<T>;
pub type HashMap<K,V> = fnv::FnvHashMap<K,V>;

pub fn default<T: Default>() -> T {
    T::default()
}

