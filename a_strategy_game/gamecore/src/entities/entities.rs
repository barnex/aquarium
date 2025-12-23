use std::fmt::write;

use crate::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct Entities3 {
    pawns: MemKeep3<Pawn3>,
    buildings: MemKeep3<Building3>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
#[repr(u8)]
enum TypeIdEnum {
    Pawn = 0,
    Building = 1,
}

impl Entities3 {
    fn new() -> Self {
        Self {
            pawns: MemKeep3::new(),
            buildings: MemKeep3::new(),
        }
    }

    pub fn insert<T>(&self, v: T) -> &T
    where
        T: Entity3T + GetStorage<T> + SetId3 + HasTypeId,
    {
        let shard = T::get_storage(self);
        let type_id = T::typeid();
        let id1 = shard.inner.insert_with_mut(v, |v, id| v.set_id3(Id3 { id, type_id }));
        let v = shard.inner.get(id1).unwrap();
        v
    }

    pub fn get<T>(&self, id: Id3) -> Option<&T>
    where
        T: Entity3T + GetStorage<T> + HasTypeId,
    {
        let shard = T::get_storage(self);
        match id.type_id == T::typeid() {
            true => shard.inner.get(id.id),
            false => None,
        }
    }

    pub fn get_dyn(&self, id: Id3) -> Option<&dyn Entity3T> {
        match id.type_id {
            TypeIdEnum::Pawn => self.pawns.get_unchecked(id.id).map(|v| v as &dyn Entity3T),
            TypeIdEnum::Building => self.buildings.get_unchecked(id.id).map(|v| v as &dyn Entity3T),
        }
    }
}

#[derive(Serialize, Deserialize)]
struct MemKeep3<T> {
    inner: MemKeep<T>,
    type_id: u8,
}

impl<T: HasTypeId> MemKeep3<T> {
    pub fn new() -> Self {
        Self {
            type_id: T::typeid() as u8,
            inner: MemKeep::new(),
        }
    }
}

impl<T> MemKeep3<T> {
    fn get_unchecked(&self, id: Id) -> Option<&T> {
        self.inner.get(id)
    }
}

pub trait HasTypeId {
    fn typeid() -> TypeIdEnum;
}

pub trait GetStorage<T> {
    fn get_storage(v: &Entities3) -> &MemKeep3<T>;
}

impl HasTypeId for Pawn3 {
    fn typeid() -> TypeIdEnum {
        TypeIdEnum::Pawn
    }
}
impl GetStorage<Pawn3> for Pawn3 {
    fn get_storage(v: &Entities3) -> &MemKeep3<Self> {
        &v.pawns
    }
}
impl SetId3 for Pawn3 {
    fn set_id3(&mut self, id: Id3) {
        self.id = id
    }
}
impl HasTypeId for Building3 {
    fn typeid() -> TypeIdEnum {
        TypeIdEnum::Building
    }
}
impl GetStorage<Building3> for Building3 {
    fn get_storage(v: &Entities3) -> &MemKeep3<Self> {
        &v.buildings
    }
}
impl SetId3 for Building3 {
    fn set_id3(&mut self, id: Id3) {
        self.id = id
    }
}

pub trait Entity3T: Debug {
    fn tile(&self) -> vec2i16;
}

// ----------------- id

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Id3 {
    id: Id,
    type_id: TypeIdEnum,
}

impl Id3 {
    pub const INVALID: Self = Self { id: Id::INVALID, type_id: TypeIdEnum::Pawn }; // << ??
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

pub trait SetId3 {
    fn set_id3(&mut self, id: Id3);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it() {
        //let p: Box<dyn Entity3T> = Box::new(Pawn::new(crate::PawnTyp::Cat, default(), crate::Team::Blue));
        let m = Entities3::new();

        let p = m.insert(Pawn3::new(PawnTyp::Cat, vec2(3, 4), Team::Blue));
        let b = m.insert(Building3::new(BuildingTyp::Farm, vec2(5, 6), Team::Red));
        println!("{p}");
        println!("{b}");

        let v1 = m.get::<Pawn3>(p.id);
        println!("{v1:?}");
        let v2 = m.get::<Building3>(p.id);
        println!("{v2:?}");

        let v3 = m.get_dyn(p.id);
        println!("{v3:?}");
    }
}
