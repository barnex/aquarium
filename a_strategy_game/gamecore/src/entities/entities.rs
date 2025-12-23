use crate::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct Entities3 {
    pub pawns: MemKeep3<Pawn>,
    pub buildings: MemKeep3<Building>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
#[repr(u8)]
enum TypeIdEnum {
    Pawn = 0,
    Building = 1,
}

impl Entities3 {
    pub fn new() -> Self {
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

    pub(crate) fn iter_dyn(&self) -> impl Iterator<Item = &dyn Entity3T> {
        self.pawns.iter_dyn().chain(self.buildings.iter_dyn())
    }

    // lifetimes??
    pub fn iter<T>(&self) -> impl Iterator<Item = &T>
    where
        T: Entity3T + GetStorage<T> + HasTypeId + 'static,
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
            TypeIdEnum::Pawn => drop(self.pawns.remove(id)),
            TypeIdEnum::Building => drop(self.buildings.remove(id)),
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

impl<T: Entity3T> MemKeep3<T> {
    fn iter_dyn(&self) -> impl Iterator<Item = &dyn Entity3T> {
        self.inner.iter().map(|v| v as &dyn Entity3T)
    }
}

pub trait HasTypeId {
    fn typeid() -> TypeIdEnum;
}

pub trait GetStorage<T> {
    fn get_storage(v: &Entities3) -> &MemKeep3<T>;
}

impl HasTypeId for Pawn {
    fn typeid() -> TypeIdEnum {
        TypeIdEnum::Pawn
    }
}
impl GetStorage<Pawn> for Pawn {
    fn get_storage(v: &Entities3) -> &MemKeep3<Self> {
        &v.pawns
    }
}
impl SetId3 for Pawn {
    fn set_id3(&mut self, id: Id3) {
        self.id = id
    }
}
impl GetId3 for Pawn {
    fn id(&self) -> Id3 {
        self.id
    }
}
impl HasTypeId for Building {
    fn typeid() -> TypeIdEnum {
        TypeIdEnum::Building
    }
}
impl GetStorage<Building> for Building {
    fn get_storage(v: &Entities3) -> &MemKeep3<Self> {
        &v.buildings
    }
}
impl SetId3 for Building {
    fn set_id3(&mut self, id: Id3) {
        self.id = id
    }
}
impl GetId3 for Building {
    fn id(&self) -> Id3 {
        self.id
    }
}

pub trait Entity3T: Debug + GetId3 + 'static {
    fn draw(&self, g: &G, out: &mut Out);
    fn tile(&self) -> vec2i16;
    fn team(&self) -> Team;
    fn can_move(&self) -> bool;
    fn bounds(&self) -> Bounds2Di16 {
        //👇 Default size
        Bounds2D::with_size(self.tile(), vec2(1, 1))
    }
}

// ----------------- id

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Id3 {
    id: Id,
    type_id: TypeIdEnum,
}

pub trait GetId3 {
    fn id(&self) -> Id3;
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
