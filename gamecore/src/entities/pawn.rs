use crate::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Pawn {
    pub tile: Cel<vec2i16>,
    pub typ: PawnTyp,
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
    //pub fn draw(&self, out: &mut Output){
    //	let sprite = match self.typ{
    //		PawnTyp::Leaf => sprite!("leaf"),
    //	}
    //}
}
