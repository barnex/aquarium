use itertools::Itertools as _;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::VecDeque;
use std::fmt::Debug;

/// VecDeque with mutable aliasing.
#[derive(Serialize, Deserialize, Clone, Default)]
#[serde(transparent)]
pub struct CDeque<T>(RefCell<VecDeque<T>>);

impl<T> CDeque<T> {
    pub fn new() -> Self {
        Self(RefCell::default())
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

    pub(crate) fn push_back(&self, v: T) {
        self.0.borrow_mut().push_back(v);
    }

    pub(crate) fn push_front(&self, v: T) {
        self.0.borrow_mut().push_front(v);
    }

    pub(crate) fn pop_back(&self) -> Option<T> {
        self.0.borrow_mut().pop_back()
    }

    pub(crate) fn pop_front(&self) -> Option<T> {
        self.0.borrow_mut().pop_front()
    }
}

impl<T: Clone> CDeque<T> {
    /// Like HashSet::iter().
    pub fn iter(&self) -> impl Iterator<Item = T> {
        // ⚠️ TODO: without cloning.
        self.0.borrow().iter().cloned().collect_vec().into_iter()
    }

    pub(crate) fn get(&self, index: usize) -> Option<T> {
        self.0.borrow_mut().get(index).cloned()
    }
}

impl<T: Copy> FromIterator<T> for CDeque<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self(RefCell::new(VecDeque::from_iter(iter)))
    }
}

impl<T: Copy + Debug> Debug for CDeque<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.borrow().fmt(f)
    }
}
