pub use crate::*;
pub use core_util::With as _;
pub use core_util::cross;
pub use gamelib::*;
pub use geometry::linterp;
pub use proc_macros::*;
pub use shell_api::*;
pub use vector::*;

pub use anyhow::{Error, Result, anyhow};
pub use itertools::Itertools as _;
pub use num_traits::AsPrimitive as _;
pub use serde::{Deserialize, Serialize};

pub use rand::{Rng, SeedableRng};
pub use rand_chacha::ChaCha8Rng;

pub type HashMap<K, V> = fnv::FnvHashMap<K, V>;

pub(crate) fn default<T: Default>() -> T {
    T::default()
}
