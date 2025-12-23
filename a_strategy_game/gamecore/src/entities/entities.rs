use crate::prelude::*;

pub struct Entities3 {
    pawns: MemKeep<Pawn>,
    buildings: MemKeep<Building>,
}

pub trait TypeId3 {
    fn typeid() -> u8;
}

impl TypeId3 for Pawn {
    fn typeid() -> u8 {
        0
    }
}

pub trait GetStorage<T> {
    fn get_storage(v: &Entities3) -> &MemKeep<T>;
}

impl GetStorage<Pawn> for Pawn {
    fn get_storage(v: &Entities3) -> &MemKeep<Pawn> {
        &v.pawns
    }
}

pub trait Entity3T {
    fn tile(&self) -> vec2i16;
}

impl Entity3T for Pawn {
    fn tile(&self) -> vec2i16 {
        self.tile.get()
    }
}

impl Entities3 {
    pub fn insert<T>(&self, v: T) -> Id
    where
        T: Entity3T + GetStorage<T> + SetId,
    {
        let shard = T::get_storage(self);
        let id1 = shard.insert_masked(v)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it() {
        let p: Box<dyn Entity3T> = Box::new(Pawn::new(crate::PawnTyp::Cat, default(), crate::Team::Blue));
    }
}
