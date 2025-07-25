pub use crate::*;
pub use num_traits::AsPrimitive;
pub use vector::*;
pub use geometry::*;

pub use engine::*;
pub use proc_macros::*;
pub use fixed_str::*;


pub use rand::{Rng, SeedableRng};
pub use serde::{Serialize, Deserialize};
pub use rand_chacha::ChaCha8Rng;
pub use std::collections::VecDeque;

pub use std::fmt::Write as _;


pub fn default<T: Default>() -> T {
    T::default()
}
