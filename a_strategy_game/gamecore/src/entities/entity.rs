use crate::prelude::*;

//struct Hex {
//    soldiers: Vec<(Base, SoldierExt)>,
//    buidlings: Vec<(Base, BuildingExt)>,
//}

// Like `Box<dyn EntityT>`.
#[derive(Serialize, Deserialize)]
pub struct EntityStorage {
    pub base: Base,
    pub ext: Ext,
}

// Like `&'g dyn EntityT`
pub struct Entity<'g> {
    pub g: &'g G,
    pub base: &'g Base,
    pub ext: &'g Ext,
}

pub trait EntityT: BaseT {
    fn tick(&self);
    fn draw(&self, out: &mut Out);
    fn g(&self) -> &G;
    fn size(&self) -> vec2u8 {
        vec2(1, 1) //👈 default, larger entities should override.
    }
    fn bounds(&self) -> Bounds2Di16 {
        Bounds2D::with_size(self.tile(), self.size().as_i16())
    }
}

//pub trait GT {
//    fn g(&self) -> &G;
//}

impl EntityStorage {
    pub(crate) fn pawn(typ: PawnTyp, team: Team, tile: vec2i16) -> EntityStorage {
        EntityStorage {
            base: Base::new(team, tile),
            ext: Ext::Pawn(Pawn2Ext::new(typ)),
        }
    }

    pub(crate) fn building(typ: BuildingTyp, team: Team, tile: Vector<i16, 2>) -> EntityStorage {
        EntityStorage {
            base: Base::new(team, tile),
            ext: Ext::Building(BuildingExt::new(typ)),
        }
    }

    pub fn as_ref<'g>(&'g self, g: &'g G) -> Entity<'g> {
        Entity { g, base: &self.base, ext: &self.ext }
    }
}

impl<'g> BaseT for Entity<'g> {
    fn base(&self) -> &Base {
        self.base
    }
}

impl<'g> EntityT for Entity<'g> {
    fn g(&self) -> &G {
        self.g
    }
    fn draw(&self, out: &mut Out) {
        match &self.ext {
            Ext::Pawn(ext) => PawnRef { g: self.g, base: &self.base, ext }.draw(out),
            Ext::Building(ext) => BuildingRef { g: self.g, base: &self.base, ext }.draw(out),
        }
    }
    fn tick(&self) {
        match &self.ext {
            Ext::Pawn(ext) => PawnRef { g: self.g, base: &self.base, ext }.tick(),
            Ext::Building(ext) => BuildingRef { g: self.g, base: &self.base, ext }.tick(),
        }
    }
}

//impl<'g> GT for Entity<'g> {
//    fn g(&self) -> &G {
//        &self.g
//    }
//}

#[derive(Serialize, Deserialize)]
pub struct Base {
    id: Id,
    tile: Cel<vec2i16>,
    health: Cel<u8>,
    team: Cel<Team>,
    sleep: Cel<u8>,
    traced: Cel<bool>,
}
impl Base {
    fn new(team: Team, tile: vec2i16) -> Self {
        Self {
            id: Id::INVALID,
            tile: tile.cel(),
            health: 1.cel(), // << TODO
            team: team.cel(),
            sleep: default(),
            traced: default(),
        }
    }
}

pub trait BaseT {
    fn base(&self) -> &Base;
    fn id(&self) -> Id {
        self.base().id
    }
    fn tile(&self) -> vec2i16 {
        self.base().tile.get()
    }
    fn health(&self) -> u8 {
        self.base().health.get()
    }
    fn team(&self) -> Team {
        self.base().team.get()
    }
    fn traced(&self) -> &Cel<bool> {
        &self.base().traced
    }
}

//impl BaseT for EntityStorage {
//    fn base(&self) -> &Base {
//        &self.base
//    }
//}
//
#[derive(Serialize, Deserialize)]
pub enum Ext {
    Pawn(Pawn2Ext),        // pawn2.rs
    Building(BuildingExt), // building2.rs
}

impl EntityStorage {
    pub fn new(tile: vec2i16, team: Team, ext: impl Into<Ext>) -> Self {
        Self {
            base: Base {
                id: Id::default(),
                tile: tile.cel(),
                health: 100.cel(),
                team: team.cel(),
                sleep: default(),
                traced: default(),
            },
            ext: ext.into(),
        }
    }
}

impl SetId for EntityStorage {
    fn set_id(&mut self, id: Id) {
        self.base.id = id
    }
}

//impl Display for EntityStorage {
//    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//        write!(f, "E{}", self.base.id)
//    }
//}

#[cfg(test)]

impl<'g> Display for Entity<'g> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}@{}", self.id(), self.tile())
    }
}
