pub use crate::*;
pub use num_traits::AsPrimitive;
pub use vector::*;
pub use geometry::*;

pub use engine::*;
pub use proc_macros::*;

pub use fixed_str::*;
pub use std::fmt::Write as _;

pub use rand::{Rng, SeedableRng};
pub use rand_chacha::ChaCha8Rng;



pub fn default<T: Default>() -> T {
    T::default()
}
