use serde::{Deserialize, Serialize};
use std::borrow::Borrow as _;
use std::borrow::BorrowMut as _;
use std::cell::RefCell;
use std::hash::Hash;

type HashSet<T> = fnv::FnvHashSet<T>;

/// Set with mutable aliasing.
#[derive(Serialize, Deserialize, Clone)]
#[serde(transparent)]
pub struct CSet<T: Eq + Hash>(RefCell<HashSet<T>>);
impl<T: Eq + Hash + Copy> CSet<T> {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    pub(crate) fn remove(&self, v: &T) {
        self.0.borrow_mut().remove(v);
    }

    pub(crate) fn insert(&self, v: T) {
        self.0.borrow_mut().insert(v);
    }
}

impl<T: Eq + Hash + Copy> FromIterator<T> for CSet<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self(RefCell::new(HashSet::from_iter(iter)))
    }
}

impl<T: Eq + Hash + Copy> Default for CSet<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}
