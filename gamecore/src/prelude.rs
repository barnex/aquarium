pub use crate::*;
pub use cel::*;
pub use geometry::*;
pub use memkeep::*;
pub use num_traits::AsPrimitive;
pub use vector::*;
pub use pathfinder::*;

pub use fixed_str::*;
pub use proc_macros::*;

pub use anyhow::{Result, anyhow};
pub use core_util::*;
pub use itertools::Itertools as _;
pub use num_enum::{IntoPrimitive, TryFromPrimitive};
pub use rand::{Rng, SeedableRng};
pub use rand_chacha::ChaCha8Rng;
pub use serde::{Deserialize, Serialize};
pub use std::convert::TryFrom;
pub use std::cell::RefCell;

pub use std::collections::VecDeque;

pub use std::cell::Cell;
pub use std::fmt::Debug;
pub use std::fmt::Write as _;

pub type HashSet<T> = fnv::FnvHashSet<T>;
pub type HashMap<K, V> = fnv::FnvHashMap<K, V>;

/// Rectangle on the screen, positions in pixels.
pub type Bounds = Bounds2D<i32>;

pub fn default<T: Default>() -> T {
    T::default()
}

/// Shorthand for returning `Some(())` from functions that return `Option<()>`  to support `?`.
pub const OK: Option<()> = Some(());
