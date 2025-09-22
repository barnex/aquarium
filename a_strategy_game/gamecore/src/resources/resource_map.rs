use crate::prelude::*;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ResourceMap {
    at_tile: RefCell<HashMap<vec2i16, ResourceTyp>>,
}

impl ResourceMap {
    pub fn at(&self, tile: vec2i16) -> Option<ResourceTyp> {
        self.at_tile.borrow().get(&tile).copied()
    }

    pub fn insert(&self, tile: vec2i16, v: ResourceTyp) -> Option<ResourceTyp> {
        self.at_tile.borrow_mut().insert(tile, v)
    }

    pub fn remove(&self, tile: vec2i16) -> Option<ResourceTyp> {
        self.at_tile.borrow_mut().remove(&tile)
    }

    /// 🪲 horrible hack: should not have to copy first
    /// Need interior mutable hashmap
    pub fn iter(&self) -> impl Iterator<Item = (vec2i16, ResourceTyp)> {
        let keys = self.at_tile.borrow().keys().copied().collect_vec();
        keys.into_iter().filter_map(|k| self.at_tile.borrow().get(&k).map(|v| (k, *v)))
    }
}
