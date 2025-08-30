use crate::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
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
    Farm = 2,
    Quarry = 3,
    // 👆 ⚠️ keep in sync!
}

impl BuildingTyp {
    pub fn all() -> impl Iterator<Item = Self> {
        let first = Self::HQ;
        let last = Self::Quarry; // 👈⚠️ keep in sync!
        ((first as u8)..=(last as u8)).map(|i| Self::try_from_primitive(i).unwrap())
    }

    pub fn sprite(&self) -> Sprite {
        use BuildingTyp::*;
        match self {
            HQ => sprite!("hq"),
            Farm => sprite!("shell_big"),
            Quarry => sprite!("quarry"),
        }
    }

}

impl Building {
    /// Building size in tiles. E.g. 3x3.
    pub fn size(&self) -> vec2u8 {
        use BuildingTyp::*;
        match self.typ {
            HQ => (3, 3),
            Farm => (2, 2),
            Quarry => (2, 2),
        }
        .into()
    }

    pub fn tile_bounds(&self) -> Bounds2Di16 {
        Bounds2D::with_size(self.tile, self.size().as_i16())
    }

    pub fn entrance(&self) -> vec2i16 {
        self.tile // TODO
    }

    pub fn new(typ: BuildingTyp, tile: impl Into<Vector<i16, 2>>) -> Self {
        Self {
            id: default(),
            typ,
            tile: tile.into(),
            workers: default(),
        }
    }
}

/// For Memkeep::insert().
impl SetId for Building {
    fn set_id(&mut self, id: Id) {
        self.id = id;
    }
}

impl Display for Building {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}{}@{}", self.typ, self.id, self.tile)
    }
}
