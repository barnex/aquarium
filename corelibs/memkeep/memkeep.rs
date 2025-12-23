use std::cell::{Cell, RefCell, UnsafeCell};
use std::fmt::{Debug, Display};
use std::marker::PhantomData;
mod serde_support;
mod with_id;
pub use with_id::*;
mod id;
pub use id::*;

/// A memory arena indexed by generational indices,
/// that hands out references (`&T`).
pub struct MemKeep<T> {
    storage: Vec<Slot<T>>,
    freelist: RefCell<Vec<u32>>,
    garbage: RefCell<Vec<u32>>,
    // TODO: high water mark for efficient iteration
    // TODO: grow storage
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

    unsafe fn insert(&self, v: T) {
        // SAFETY: We just check that the slot is free, so no other pointers exist.
        // debug_assert double-checks this.
        let ptr = unsafe { &mut *self.value.get() };
        debug_assert!(ptr.is_none());
        *ptr = Some(v);
    }
}

impl<T> MemKeep<T> {
    pub fn new() -> Self {
        let n = 1024;
        Self {
            storage: (0..n).map(|_| Slot::initial()).collect(),
            freelist: RefCell::new((0..n).rev().collect()),
            garbage: RefCell::new(Vec::new()),
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

    pub fn get_maybe(&self, id: Option<Id>) -> Option<&T> {
        id.and_then(|id| self.get(id))
    }

    pub fn insert_without_setting_id(&self, v: T) -> Id {
        let (id, slot) = self._prepare_slot();
        unsafe { slot.insert(v) };
        id
    }

    pub fn insert_with_mut(&self, v: T, f: impl FnOnce(&mut T, Id)) -> Id {
        let (id, slot) = self._prepare_slot();
        let mut v = v;
        f(&mut v, id);
        unsafe { slot.insert(v) };
        id
    }

    fn _prepare_slot(&self) -> (Id, &Slot<T>) {
        let index = self.freelist.borrow_mut().pop().expect("MemKeep full");
        debug_assert!(self.storage[index as usize].not_deleted.get() == false);

        let slot = &self.storage[index as usize];
        let generation = slot.generation.get().wrapping_add(1);

        slot.generation.set(generation);

        slot.not_deleted.set(true);

        let id = Id { index, generation };
        (id, slot)
    }

    // For deserialize only
    // !! Must rebuild freelist after.
    fn _insert_at(&mut self, id: Id, v: T) {
        let slot = &mut self.storage[id.index as usize];
        let generation = id.generation;
        slot.generation.set(generation);
        slot.not_deleted.set(true);
        slot.value = UnsafeCell::new(Some(v));
    }

    pub fn remove(&self, id: Id) -> Option<&T> {
        debug_assert!((id.index as usize) < self.storage.len(), "Id index out of bounds, can only happen if an invalid Id is passed (e.g. obtained from a different MemKeep)");

        let slot = &self.storage.get(id.index as usize)?; // 👈 ignore out-of-bounds in release builds
        if slot.generation.get() == id.generation && slot.not_deleted.get() {
            // SAFETY: Handing out shared references. Only mutated in `gc()` where `&mut self` guarantees all shared references are dropped.
            slot.not_deleted.set(false);
            self.garbage.borrow_mut().push(id.index);
            let v: &Option<T> = unsafe { &*slot.value.get() };
            Some(v.as_ref().unwrap())
        } else {
            None
        }
    }

    pub fn enumerate(&self) -> impl Iterator<Item = (Id, &T)> {
        // SAFETY: Only shared references can be handed out at this moment.
        // Only mutated in `gc()` where `&mut self` guarantees all shared references are dropped.
        self.storage.iter().enumerate().filter(|(_, slot)| slot.not_deleted.get() && unsafe { &*slot.value.get() }.is_some()).map(|(i, slot)| {
            (
                Id {
                    index: i as u32,
                    generation: slot.generation.get(),
                },
                unsafe { &*slot.value.get() }.as_ref().unwrap(),
            )
        })
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.enumerate().map(|(_, v)| v)
    }

    pub fn gc(&mut self) {
        for index in self.garbage.get_mut().drain(..) {
            let slot = &mut self.storage[index as usize];
            if !slot.not_deleted.get() {
                slot.value.get_mut().take();
                self.freelist.get_mut().push(index as u32);
            }
        }
    }
}

#[cfg(test)]
mod memkeep_test {

    use super::*;
    use googletest::prelude::*;

    #[gtest]
    fn unique_ids() {
        let m = MemKeep::<&'static str>::new();

        let a = m.insert_without_setting_id("a");
        let b = m.insert_without_setting_id("b");
        let c = m.insert_without_setting_id("c");

        // references are unique
        expect_ne!(a, b);
        expect_ne!(a, c);
        expect_ne!(b, c);
    }

    #[gtest]
    fn get() {
        let m = MemKeep::<&'static str>::new();

        let a = m.insert_without_setting_id("a");
        let b = m.insert_without_setting_id("b");
        let c = m.insert_without_setting_id("c");

        expect_eq!(m.get(a), Some(&"a"));
        expect_eq!(m.get(b), Some(&"b"));
        expect_eq!(m.get(c), Some(&"c"));
    }

    #[gtest]
    fn enumerate() {
        let mut m = MemKeep::<&'static str>::new();

        let a = m.insert_without_setting_id("a");
        let b = m.insert_without_setting_id("b");
        let c = m.insert_without_setting_id("c");
        let d = m.insert_without_setting_id("d");
        let e = m.insert_without_setting_id("e");

        // exercise garbage, collected and alive
        m.remove(c);
        m.gc();
        m.remove(d);

        assert_eq!(m.get(a), Some(&"a"));
        assert_eq!(m.get(b), Some(&"b"));
        assert_eq!(m.get(c), None);
        assert_eq!(m.get(d), None);
        assert_eq!(m.get(e), Some(&"e"));

        expect_eq!(m.enumerate().collect::<Vec<_>>(), vec![(a, &"a"), (b, &"b"), (e, &"e"),])
    }

    #[gtest]
    fn serde() {
        let mut m = MemKeep::<String>::new();
        let a = m.insert_without_setting_id("a".into());
        let b = m.insert_without_setting_id("b".into());
        let c = m.insert_without_setting_id("c".into());
        let d = m.insert_without_setting_id("d".into());
        let e = m.insert_without_setting_id("e".into());
        // exercise garbage, collected and alive
        m.remove(c);
        m.gc();
        m.remove(d);
        assert_eq!(m.get(a), Some(&"a".to_string()));
        assert_eq!(m.get(b), Some(&"b".to_string()));
        assert_eq!(m.get(c), None);
        assert_eq!(m.get(d), None);
        assert_eq!(m.get(e), Some(&"e".to_string()));

        let serialized = ron::to_string(&m).unwrap();

        let de: MemKeep<String> = ron::from_str(&serialized).unwrap();

        let m = m.enumerate().collect::<Vec<_>>();
        let de = de.enumerate().collect::<Vec<_>>();

        expect_eq!(m, de);
    }

    #[gtest]
    fn deserialize_ron() {
        let m: MemKeep<String> = ron::from_str("[((0,1),\"a\"),((1,1),\"b\"),((4,2),\"e\")]").unwrap();
        expect_eq!(
            m.enumerate().collect::<Vec<_>>(),
            vec![(Id { index: 0, generation: 1 }, &"a".to_string()), (Id { index: 1, generation: 1 }, &"b".to_string()), (Id { index: 4, generation: 2 }, &"e".to_string()),]
        )
    }

    #[gtest]
    fn remove() {
        let m = MemKeep::<&'static str>::new();

        let a = m.insert_without_setting_id("a");
        let b = m.insert_without_setting_id("b");
        let c = m.insert_without_setting_id("c");

        expect_eq!(m.get(a), Some(&"a"));
        expect_eq!(m.get(b), Some(&"b"));
        expect_eq!(m.get(c), Some(&"c"));

        expect_eq!(m.remove(b), Some(&"b"));

        expect_eq!(m.get(a), Some(&"a"));
        expect_eq!(m.get(b), None); // 👈
        expect_eq!(m.get(c), Some(&"c"));
    }

    #[gtest]
    /// Runs out of storage when gc does not work.
    fn gc() {
        let mut m = MemKeep::<&'static str>::new();

        let a1 = m.insert_without_setting_id("hello, computer!");
        m.remove(a1);
        m.gc();

        for _ in 0..20 {
            let a = m.insert_without_setting_id("hello, computer!");
            expect_eq!(a.index, a1.index);
            m.remove(a);
            m.gc();
        }
    }
}
