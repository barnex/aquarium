use std::cell::{Cell, RefCell, UnsafeCell};
use std::fmt::{Debug, Display};

/// A memory arena indexed by generational indices,
/// that hands out references (`&T`).
pub struct MemKeep<T> {
    storage: Vec<Slot<T>>,
    freelist: RefCell<Vec<u32>>,
}

struct Slot<T> {
    generation: Cell<u32>,
    not_deleted: Cell<bool>,
    value: UnsafeCell<Option<T>>,
}

impl<T> Slot<T> {
    fn initial() -> Self {
        Self {
            generation: 0.into(),
            not_deleted: false.into(),
            value: UnsafeCell::new(None),
        }
    }
}

impl<T> MemKeep<T> {
    pub fn new() -> Self {
        let n = 1024;
        Self {
            storage: (0..n).map(|_| Slot::initial()).collect(),
            freelist: RefCell::new((0..n).rev().collect()),
        }
    }

    #[inline]
    #[track_caller]
    pub fn get(&self, id: Id) -> Option<&T> {
        debug_assert!((id.index as usize) < self.storage.len(), "Id index out of bounds, can only happen if an invalid Id is passed (e.g. obtained from a different MemKeep)");

        let slot = &self.storage.get(id.index as usize)?; // 👈 ignore out-of-bounds in release builds
        if slot.generation.get() == id.generation && slot.not_deleted.get() {
            // SAFETY: Handing out shared references. Only mutated in `gc()` where `&mut self` guarantees all shared references are dropped.
            let v: &Option<T> = unsafe { &*slot.value.get() };
            Some(v.as_ref().unwrap())
        } else {
            None
        }
    }

    pub fn insert(&self, v: T) -> Id {
        let index = self.freelist.borrow_mut().pop().expect("MemKeep full");
        debug_assert!(self.storage[index as usize].not_deleted.get() == false);

        let slot = &self.storage[index as usize];
        let generation = slot.generation.get().wrapping_add(1);
        slot.generation.set(generation);
        slot.not_deleted.set(true);
        {
            // SAFETY: We just check that the slot is free, so no other pointers exist.
            // debug_assert double-checks this.
            let ptr = unsafe { &mut *slot.value.get() };
            debug_assert!(ptr.is_none());
            *ptr = Some(v);
            // drop ptr
        }
        Id { index, generation }
    }

    pub fn remove(&self, id: Id) -> Option<&T> {
        debug_assert!((id.index as usize) < self.storage.len(), "Id index out of bounds, can only happen if an invalid Id is passed (e.g. obtained from a different MemKeep)");

        let slot = &self.storage.get(id.index as usize)?; // 👈 ignore out-of-bounds in release builds
        if slot.generation.get() == id.generation && slot.not_deleted.get() {
            // SAFETY: Handing out shared references. Only mutated in `gc()` where `&mut self` guarantees all shared references are dropped.
            slot.not_deleted.set(false);
            let v: &Option<T> = unsafe { &*slot.value.get() };
            Some(v.as_ref().unwrap())
        } else {
            None
        }
    }

    pub fn gc(&mut self) {

    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Id {
    index: u32,
    generation: u32,
}

impl Debug for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Self as Display>::fmt(self, f)
    }
}

impl Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:08x}.{:08x}", self.index, self.generation)
    }
}

#[cfg(test)]
mod memkeep_test {

    use super::*;
    use googletest::prelude::*;

    #[gtest]
    fn unique_ids() {
        let m = MemKeep::<&'static str>::new();

        let a = m.insert("a");
        let b = m.insert("b");
        let c = m.insert("c");

        // references are unique
        expect_ne!(a, b);
        expect_ne!(a, c);
        expect_ne!(b, c);
    }

    #[gtest]
    fn get() {
        let m = MemKeep::<&'static str>::new();

        let a = m.insert("a");
        let b = m.insert("b");
        let c = m.insert("c");

        expect_eq!(m.get(a), Some(&"a"));
        expect_eq!(m.get(b), Some(&"b"));
        expect_eq!(m.get(c), Some(&"c"));
    }

    #[gtest]
    fn remove() {
        let m = MemKeep::<&'static str>::new();

        let a = m.insert("a");
        let b = m.insert("b");
        let c = m.insert("c");

        expect_eq!(m.get(a), Some(&"a"));
        expect_eq!(m.get(b), Some(&"b"));
        expect_eq!(m.get(c), Some(&"c"));

        expect_eq!(m.remove(b), Some(&"b"));

        expect_eq!(m.get(a), Some(&"a"));
        expect_eq!(m.get(b), None); // 👈
        expect_eq!(m.get(c), Some(&"c"));
    }
}
