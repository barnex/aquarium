use crate::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct Building {
    pub typ: BuildingTyp,
    pub tile: vec2i16,
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

impl Building {
    /// Building size in tiles. E.g. 3x3.
    pub fn size(&self) -> vec2u8 {
        use BuildingTyp::*;
        match self.typ {
            HQ => (3, 3),
        }
        .into()
    }
    
    pub fn tile_bounds(&self) -> Bounds2Di16{
        Bounds2D::with_size(self.tile, self.size().as_i16())
    }
}
