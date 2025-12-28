use std::hash::Hash;

use crate::prelude::*;

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct CMap<K: Hash + Eq, V>(pub RefCell<HashMap<K, V>>);

impl<K: Hash + Eq + Copy, V: Copy> CMap<K, V> {
    pub fn insert(&self, k: K, v: V) {
        self.0.borrow_mut().insert(k, v);
    }

    pub fn remove(&self, k: K) {
        self.0.borrow_mut().remove(&k);
    }

    pub fn get(&self, k: K) -> Option<V> {
        self.0.borrow_mut().get(&k).copied()
    }
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Renewables {
    pub at_tile: CMap<vec2i16, u8>,
}
