use crate::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Pawn {
    pub id: Id,
    pub typ: PawnTyp,
    pub tile: Cel<vec2i16>,
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash, TryFromPrimitive, Debug)]
#[repr(u8)]
pub enum PawnTyp {
    Leaf = 1,
    Pollen = 2,
    Cat = 3,
    Crablet = 4,
    // ⚠️ update `all()` below!
}
impl PawnTyp {
    // ⚠️ keep in sync!
    pub fn all() -> impl Iterator<Item = Self> {
        ((Self::Leaf as u8)..=(Self::Crablet as u8)).map(|i| Self::try_from_primitive(i).unwrap())
    }
}

impl PawnTyp {
    pub fn sprite(&self) -> Sprite {
        match self {
            PawnTyp::Leaf => sprite!("leaf"),
            PawnTyp::Pollen => sprite!("pollen"),
            PawnTyp::Cat => sprite!("kit4"),
            PawnTyp::Crablet => sprite!("ferris"),
        }
    }
}

impl Pawn {
    pub fn new(typ: PawnTyp, tile: vec2i16) -> Self {
        Self { tile: tile.cel(), typ, id: Id::default() }
    }
    
    pub fn bounds(&self) -> Bounds2Di{
        Bounds2D::with_size(self.tile.pos(), vec2::splat(TILE_ISIZE)) 
    }
}

impl SetId for Pawn {
    fn set_id(&mut self, id: Id) {
        self.id = id;
    }
}
