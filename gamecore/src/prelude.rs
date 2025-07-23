pub use crate::*;

pub use engine::*;
pub use proc_macros::*;

pub use std::fmt::Write as _;
pub use fixed_str::*;

pub fn default<T: Default>() -> T {
    T::default()
}
