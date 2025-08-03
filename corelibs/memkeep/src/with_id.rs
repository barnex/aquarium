/// Game Objects (implementing `SetId`) have their `Id` automatically updated upon insertion.
/// E.g.
/// ```
/// let m = MemKeep::new();
/// m.insert(Monster::new()); // sets monster Id.
/// ...
/// now_use(monster.id);
/// ```
use crate::*;

/// Allows setting an object's `Id` upon insertion.
pub trait SetId {
    fn set_id(&mut self, id: Id);
}

impl<T: SetId> MemKeep<T> {
    /// Insert and store the Id by calling `v.set_id(id)`.
    pub fn insert(&self, v: T) -> Id {
        let (id, slot) = self._prepare_slot();
        let mut v = v;
        v.set_id(id);
        unsafe { slot.insert(v) };
        id
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use googletest::prelude::*;

    #[gtest]
    fn insert_with_id() {
        #[derive(Debug, PartialEq, Eq)]
        struct Person {
            id: Id,
            name: &'static str,
        }

        impl Person {
            fn new(name: &'static str) -> Self {
                Self { name, id: Id::default() }
            }
        }

        impl SetId for Person {
            fn set_id(&mut self, id: Id) {
                self.id = id
            }
        }

        let mut m = MemKeep::new();

        let alice = Person::new("alice");
        let bob = Person::new("bob");

        let a = m.insert(alice);
        let b = m.insert(bob);

        assert!(a.is_valid());
        assert!(b.is_valid());

        expect_eq!(m.get(a).map(|v| v.name), Some("alice"));
        expect_eq!(m.get(b).map(|v| v.name), Some("bob"));

        expect_eq!(m.get(a).map(|v| v.id), Some(a));
        expect_eq!(m.get(b).map(|v| v.id), Some(b));

        m.gc();

        expect_eq!(m.get(a).map(|v| v.name), Some("alice"));
        expect_eq!(m.get(b).map(|v| v.name), Some("bob"));

        expect_eq!(m.get(a).map(|v| v.id), Some(a));
        expect_eq!(m.get(b).map(|v| v.id), Some(b));
    }
}
