pub use crate::*;
pub use core_util::With as _;
pub use gamelib::*;
pub use proc_macros::*;
pub use shell_api::*;
pub use vector::*;

pub use anyhow::{Error, Result, anyhow};
pub use itertools::Itertools as _;
pub use serde::{Deserialize, Serialize};

pub type HashMap<K, V> = fnv::FnvHashMap<K, V>;

pub(crate) fn default<T: Default>() -> T {
    T::default()
}
