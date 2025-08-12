use itertools::Itertools as _;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::hash::Hash;

type HashSet<T> = fnv::FnvHashSet<T>;

/// Set with mutable aliasing.
#[derive(Serialize, Deserialize, Clone)]
#[serde(transparent)]
pub struct CSet<T: Eq + Hash>(RefCell<HashSet<T>>);
impl<T: Eq + Hash + Copy> CSet<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn remove(&self, v: &T) {
        self.0.borrow_mut().remove(v);
    }

    pub fn insert(&self, v: T) {
        self.0.borrow_mut().insert(v);
    }

    pub fn iter(&self) -> impl Iterator<Item = T> {
        // ⚠️ TODO: without cloning.
        self.0.borrow().iter().copied().collect_vec().into_iter()
    }

    pub fn len(&self) -> usize {
        self.0.borrow().len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.borrow().is_empty()
    }

    pub fn clear(&self) {
        self.0.borrow_mut().clear();
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
