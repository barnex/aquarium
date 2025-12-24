use crate::prelude::*;
use memkeep::MemKeep;

#[derive(Serialize, Deserialize)]
pub struct Entities {
    pawns: MemKeep3<Pawn>,
    buildings: MemKeep3<Building>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
#[repr(u8)]
pub enum EntityType {
    Pawn = 0,
    Building = 1,
}

impl Entities {
    pub fn new() -> Self {
        Self {
            pawns: MemKeep3::new(),
            buildings: MemKeep3::new(),
        }
    }

    pub fn insert<T>(&self, v: T) -> &T
    where
        T: EntityT + HasTypeId,
    {
        let shard = T::get_storage(self);
        let type_id = T::typeid();
        let id1 = shard.inner.insert_with_mut(v, |v, id| v.set_id3(Id { id, type_id }));
        let v = shard.inner.get(id1).unwrap();
        v
    }

    pub fn get<T>(&self, id: Id) -> Option<&T>
    where
        T: EntityT + HasTypeId,
    {
        let shard = T::get_storage(self);
        match id.type_id == T::typeid() {
            true => shard.inner.get(id.id),
            false => None,
        }
    }

    pub fn get_dyn(&self, id: Id) -> Option<Entity> {
        match id.type_id {
            EntityType::Pawn => self.pawns.get_unchecked(id).map(|v| Entity::from(v)),
            EntityType::Building => self.buildings.get_unchecked(id).map(|v| Entity::from(v)),
        }
    }

    pub(crate) fn iter_dyn(&self) -> impl Iterator<Item = Entity> {
        self.pawns.iter().map(|v| Entity::from(v)).chain(self.buildings.iter().map(|v| Entity::from(v)))
    }

    pub fn iter<T>(&self) -> impl Iterator<Item = &T>
    where
        T: EntityT + HasTypeId,
    {
        let shard = T::get_storage(self);
        shard.iter()
    }

    pub(crate) fn gc(&mut self) {
        self.pawns.gc();
        self.buildings.gc();
    }

    pub fn remove(&self, id: Id) {
        match id.type_id {
            EntityType::Pawn => drop(self.pawns.remove(id)),
            EntityType::Building => drop(self.buildings.remove(id)),
        }
    }
}

#[derive(Serialize, Deserialize)]
struct MemKeep3<T> {
    inner: MemKeep<T>,
    type_id: u8,
}

impl<T> MemKeep3<T> {
    // dont' check the Id3 type tag, for when we already know we're indexing in the correct shard.
    fn get_unchecked(&self, id: Id) -> Option<&T> {
        debug_assert_eq!(id.type_id as u8, self.type_id);
        self.inner.get(id.id)
    }

    pub fn get(&self, id: Id) -> Option<&T> {
        debug_assert_eq!(id.type_id as u8, self.type_id);
        self.inner.get(id.id)
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.inner.iter()
    }
    fn remove(&self, id: Id) -> Option<&T> {
        debug_assert_eq!(id.type_id as u8, self.type_id);
        self.inner.remove(id.id)
    }
}

impl<T: HasTypeId> MemKeep3<T> {
    pub fn new() -> Self {
        Self {
            type_id: T::typeid() as u8,
            inner: MemKeep::new(),
        }
    }

    fn gc(&mut self) {
        self.inner.gc();
    }
}

impl<T: EntityT> MemKeep3<T> {
    //fn iter_dyn(&self) -> impl Iterator<Item = Entity> {
    //    self.inner.iter().map(|v| Entity::from_dyn(v))
    //}
}

pub trait HasTypeId: Sized {
    fn typeid() -> EntityType;
    fn get_storage(v: &Entities) -> &MemKeep3<Self>;
}

impl HasTypeId for Pawn {
    fn typeid() -> EntityType {
        EntityType::Pawn
    }
    fn get_storage(v: &Entities) -> &MemKeep3<Self> {
        &v.pawns
    }
}
impl HasTypeId for Building {
    fn typeid() -> EntityType {
        EntityType::Building
    }
    fn get_storage(v: &Entities) -> &MemKeep3<Self> {
        &v.buildings
    }
}

// ----------------- id

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Id {
    id: memkeep::Id,
    type_id: EntityType,
}

impl Id {
    pub const INVALID: Self = Self {
        id: memkeep::Id::INVALID,
        type_id: EntityType::Pawn,
    }; // << ??
}

impl Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02x}:{}", self.type_id as u8, self.id)
    }
}

impl Debug for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

pub trait HasId3 {
    fn set_id3(&mut self, id: Id);
    //fn id(&self) -> Id;
}

#[cfg(test)]
mod test {
    use super::*;
    use googletest::prelude::*;

    #[gtest]
    fn it() {
        //let p: Box<dyn Entity3T> = Box::new(Pawn::new(crate::PawnTyp::Cat, default(), crate::Team::Blue));
        let m = Entities::new();

        let pawn = m.insert(Pawn::new(PawnTyp::Cat, vec2(3, 4), Team::Blue));
        let building = m.insert(Building::new(BuildingTyp::Farm, vec2(5, 6), Team::Red));

        expect_true!(pawn.id() != Id::INVALID);
        expect_true!(building.id() != Id::INVALID);

        expect_true!(m.get::<Pawn>(pawn.id()).is_some());
        expect_true!(m.get::<Building>(pawn.id()).is_none());

        expect_true!(m.get::<Pawn>(building.id()).is_none());
        expect_true!(m.get::<Building>(building.id()).is_some());

        //let v3 = m.get_dyn(p.id);
        //println!("{v3:?}");

        //for v in m.iter_dyn() {
        //    println!("i: {v:?}")
        //}
    }
}
