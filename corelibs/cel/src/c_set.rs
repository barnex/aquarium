use itertools::Itertools as _;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::fmt::Debug;
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

    /// Like HashSet::remove().
    pub fn remove(&self, v: &T) -> bool {
        self.0.borrow_mut().remove(v)
    }

    /// Like HashSet::insert().
    pub fn insert(&self, v: T) {
        self.0.borrow_mut().insert(v);
    }

    /// Like HashSet::iter().
    pub fn iter(&self) -> impl Iterator<Item = T> {
        // ⚠️ TODO: without cloning.
        self.0.borrow().iter().copied().collect_vec().into_iter()
    }

    /// Like HashSet::len.
    pub fn len(&self) -> usize {
        self.0.borrow().len()
    }

    /// Like HashSet::is_empty.
    pub fn is_empty(&self) -> bool {
        self.0.borrow().is_empty()
    }

    /// Like HashSet::clear.
    pub fn clear(&self) {
        self.0.borrow_mut().clear();
    }

    /// Like HashSet::extend.
    pub fn extend(&self, iter: impl IntoIterator<Item = T>) {
        self.0.borrow_mut().extend(iter);
    }

    /// Like HashSet::contains.
    pub fn contains(&self, v: T) -> bool {
        self.0.borrow().contains(&v)
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

impl<T: Eq + Hash + Copy + Debug> Debug for CSet<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.borrow().fmt(f)
    }
}
