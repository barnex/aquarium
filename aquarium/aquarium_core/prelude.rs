pub use crate::*;
pub use gamelib::*;
pub use proc_macros::*;
pub use shell_api::*;
pub use vector::*;
pub use core_util::With as _;

pub use serde::{Deserialize, Serialize};
pub use itertools::Itertools as _;
pub use anyhow::{Result, Error, anyhow};

pub(crate) fn default<T: Default>() -> T {
    T::default()
}
