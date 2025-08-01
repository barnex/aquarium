use crate::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct Building {
    pub typ: BuildingTyp,
    pub tile: vec2i,
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum BuildingTyp {
    HQ = 1,
}

impl BuildingTyp {
    pub fn sprite(&self) -> Sprite {
        use BuildingTyp::*;
        match self {
            HQ => sprite!("hq"),
        }
    }
}
