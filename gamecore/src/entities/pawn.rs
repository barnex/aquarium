use crate::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct Pawn {
    pub tile: vec2i,
    pub typ: PawnTyp,
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash, TryFromPrimitive, Debug)]
#[repr(u8)]
pub enum PawnTyp {
    Leaf = 0,
    Pollen = 1,
}

impl PawnTyp {
    // ⚠️ Must keep in sync with enum
    pub const NUM_TYP: usize = 2;
	
    pub fn sprite(&self) -> Sprite {
        match self {
            PawnTyp::Leaf => sprite!("leaf"),
            PawnTyp::Pollen => sprite!("pollen"),
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
