use serde::de::{SeqAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use std::cell::{Cell, RefCell, UnsafeCell};
use std::fmt::{Debug, Display};
use std::marker::PhantomData;

/// A memory arena indexed by generational indices,
/// that hands out references (`&T`).
pub struct MemKeep<T> {
    storage: Vec<Slot<T>>,
    freelist: RefCell<Vec<u32>>,
    garbage: RefCell<Vec<u32>>,
    // TODO: high water mark for efficient iteration
    // TODO: grow storage
}

impl<T> Serialize for MemKeep<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.enumerate().collect::<Vec<_>>().serialize(serializer)
    }
}

impl<'de, T> Deserialize<'de> for MemKeep<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MyWrapperVisitor<T> {
            marker: PhantomData<fn() -> MemKeep<T>>,
        }

        impl<'de, T> Visitor<'de> for MyWrapperVisitor<T>
        where
            T: Deserialize<'de>,
        {
            type Value = MemKeep<T>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a list of items for MyWrapper")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let mut m = MemKeep::new();

                while let Some((id, v)) = seq.next_element()? {
                    m._insert_at(id, v)
                }

                for (i, slot) in m.storage.iter_mut().enumerate().rev() {
                    if !slot.not_deleted.get() {
                        debug_assert!(slot.value.get_mut().is_none());
                        m.freelist.get_mut().push(i as u32);
                    }
                }

                Ok(m)
            }
        }

        deserializer.deserialize_seq(MyWrapperVisitor { marker: PhantomData })
    }
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

#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
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
    fn enumerate() {
        let mut m = MemKeep::<&'static str>::new();

        let a = m.insert("a");
        let b = m.insert("b");
        let c = m.insert("c");
        let d = m.insert("d");
        let e = m.insert("e");

        // exercise garbage, collected and alive
        m.remove(c);
        m.gc();
        m.remove(d);

        expect_eq!(m.get(a), Some(&"a"));
        expect_eq!(m.get(b), Some(&"b"));
        expect_eq!(m.get(c), None);
        expect_eq!(m.get(d), None);
        expect_eq!(m.get(e), Some(&"e"));

        expect_eq!(m.enumerate().collect::<Vec<_>>(), vec![(a, &"a"), (b, &"b"), (e, &"e"),])
    }

    #[gtest]
    fn serde() {
        let mut m = MemKeep::<String>::new();
        let a = m.insert("a".into());
        let b = m.insert("b".into());
        let c = m.insert("c".into());
        let d = m.insert("d".into());
        let e = m.insert("e".into());
        // exercise garbage, collected and alive
        m.remove(c);
        m.gc();
        m.remove(d);
        expect_eq!(m.get(a), Some(&"a".to_string()));
        expect_eq!(m.get(b), Some(&"b".to_string()));
        expect_eq!(m.get(c), None);
        expect_eq!(m.get(d), None);
        expect_eq!(m.get(e), Some(&"e".to_string()));
        
        let bytes = ron::to_string(&m).unwrap();

        //expect_eq!(bytes, "");

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

    #[gtest]
    /// Runs out of storage when gc does not work.
    fn gc() {
        let mut m = MemKeep::<&'static str>::new();

        let a1 = m.insert("hello, computer!");
        m.remove(a1);
        m.gc();

        for _ in 0..20 {
            let a = m.insert("hello, computer!");
            expect_eq!(a.index, a1.index);
            m.remove(a);
            m.gc();
        }
    }
}
