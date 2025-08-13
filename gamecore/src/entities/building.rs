use crate::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct Building {
    pub id: Id,
    pub typ: BuildingTyp,
    pub tile: vec2i16,
    pub workers: CSet<Id>,
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash, TryFromPrimitive, Debug)]
#[repr(u8)]
pub enum BuildingTyp {
    HQ = 1,
    // 👆 ⚠️ keep in sync!
}

impl BuildingTyp {
    pub fn all() -> impl Iterator<Item = Self> {
        let first = Self::HQ;
        let last = Self::HQ; // 👈⚠️ keep in sync!
        ((first as u8)..=(last as u8)).map(|i| Self::try_from_primitive(i).unwrap())
    }

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

    pub fn tile_bounds(&self) -> Bounds2Di16 {
        Bounds2D::with_size(self.tile, self.size().as_i16())
    }

    pub fn entrance(&self) -> vec2i16 {
        self.tile // TODO
    }

    pub(crate) fn new(typ: BuildingTyp, tile: Vector<i16, 2>) -> Self {
        Self { id: default(), typ, tile, workers: default() }
    }
}

impl SetId for Building {
    fn set_id(&mut self, id: Id) {
        self.id = id;
    }
}
