use crate::prelude::*;

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
        T: Entity + HasTypeId,
    {
        let shard = T::get_storage(self);
        let type_id = T::typeid();
        let id1 = shard.inner.insert_with_mut(v, |v, id| v.set_id3(Id3 { id, type_id }));
        let v = shard.inner.get(id1).unwrap();
        v
    }

    pub fn get<T>(&self, id: Id3) -> Option<&T>
    where
        T: Entity + HasTypeId,
    {
        let shard = T::get_storage(self);
        match id.type_id == T::typeid() {
            true => shard.inner.get(id.id),
            false => None,
        }
    }

    pub fn get_dyn(&self, id: Id3) -> Option<&dyn Entity> {
        match id.type_id {
            EntityType::Pawn => self.pawns.get_unchecked(id.id).map(|v| v as &dyn Entity),
            EntityType::Building => self.buildings.get_unchecked(id.id).map(|v| v as &dyn Entity),
        }
    }

    pub(crate) fn iter_dyn(&self) -> impl Iterator<Item = &dyn Entity> {
        self.pawns.iter_dyn().chain(self.buildings.iter_dyn())
    }

    pub fn iter<T>(&self) -> impl Iterator<Item = &T>
    where
        T: Entity + HasTypeId,
    {
        let shard = T::get_storage(self);
        shard.iter()
    }

    pub(crate) fn gc(&mut self) {
        self.pawns.gc();
        self.buildings.gc();
    }

    pub fn remove(&self, id: Id3) {
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
        self.inner.get(id)
    }

    pub fn get(&self, id: Id3) -> Option<&T> {
        debug_assert_eq!(id.type_id as u8, self.type_id);
        self.inner.get(id.id)
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.inner.iter()
    }
    fn remove(&self, id: Id3) -> Option<&T> {
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

impl<T: Entity> MemKeep3<T> {
    fn iter_dyn(&self) -> impl Iterator<Item = &dyn Entity> {
        self.inner.iter().map(|v| v as &dyn Entity)
    }
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
impl HasId3 for Pawn {
    fn set_id3(&mut self, id: Id3) {
        self.id = id
    }
    fn id(&self) -> Id3 {
        self.id
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
impl HasId3 for Building {
    fn set_id3(&mut self, id: Id3) {
        self.id = id
    }
    fn id(&self) -> Id3 {
        self.id
    }
}

// ----------------- id

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Id3 {
    id: Id,
    type_id: EntityType,
}

impl Id3 {
    pub const INVALID: Self = Self { id: Id::INVALID, type_id: EntityType::Pawn }; // << ??
}

impl Display for Id3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02x}:{}", self.type_id as u8, self.id)
    }
}

impl Debug for Id3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

pub trait HasId3 {
    fn set_id3(&mut self, id: Id3);
    fn id(&self) -> Id3;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it() {
        //let p: Box<dyn Entity3T> = Box::new(Pawn::new(crate::PawnTyp::Cat, default(), crate::Team::Blue));
        let m = Entities::new();

        let p = m.insert(Pawn::new(PawnTyp::Cat, vec2(3, 4), Team::Blue));
        let b = m.insert(Building::new(BuildingTyp::Farm, vec2(5, 6), Team::Red));
        println!("{p}");
        println!("{b}");

        let v1 = m.get::<Pawn>(p.id);
        println!("{v1:?}");
        let v2 = m.get::<Building>(p.id);
        println!("{v2:?}");

        let v3 = m.get_dyn(p.id);
        println!("{v3:?}");

        for v in m.iter_dyn() {
            println!("i: {v:?}")
        }
    }
}
