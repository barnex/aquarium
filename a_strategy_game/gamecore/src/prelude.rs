pub use crate::*;
pub use cel::*;
pub use gamelib::*;
pub use geometry::*;
pub use num_traits::AsPrimitive;
pub use pathfinder::*;
pub use shell_api::*;
pub use vector::*;

pub use fixed_str::*;
pub use proc_macros::*;

pub use anyhow::{Result, anyhow};
pub use core_util::*;
pub use itertools::Itertools as _;
pub use num_enum::TryFromPrimitive;
pub use rand::{Rng, SeedableRng};
pub use rand_chacha::ChaCha8Rng;
pub use serde::{Deserialize, Serialize};
pub use std::any::Any;
pub use std::cell::RefCell;
pub use std::convert::TryFrom;
pub use std::fmt;
pub use std::iter::zip;
pub use std::num::NonZeroU8;
pub use std::ops::Deref;

pub use std::collections::VecDeque;

pub use std::cell::Cell;
pub use std::fmt::Write as _;
pub use std::fmt::{Debug, Display};
pub use std::ops::{Range, RangeInclusive};

pub type HashSet<T> = fnv::FnvHashSet<T>;
pub type HashMap<K, V> = fnv::FnvHashMap<K, V>;
pub type SmallVec<T, const N: usize> = smallvec::SmallVec<[T; N]>;

/// Rectangle on the screen, positions in pixels.
pub type Bounds = Bounds2D<i32>;

pub fn default<T: Default>() -> T {
    T::default()
}

/// Shorthand for returning `Some(())` from functions that return `Option<()>`  to support `?`.
pub type Status = Option<()>;
pub const OK: Option<()> = Some(());
pub const FAIL: Option<()> = None;
