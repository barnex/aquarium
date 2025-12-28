use crate::prelude::*;

#[derive(Copy, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash, TryFromPrimitive, Debug)]
#[repr(u8)]
pub enum Tile {
    #[default]
    Dunes = 1,
    Mountains = 2,
    Sand = 3,
    Snow = 4,
    Water = 5,
    Block = 6,
    Canal = 7,
    Farmland = 8,
    Road = 9,
    GreyStone = 10,
    // WARNING: ‼️ keep in sync with `all`
}

impl Tile {
    // WARNING: ‼️ keep in sync with enum values.
    pub fn all() -> impl Iterator<Item = Self> {
        debug_assert_eq!(size_of::<Option<Self>>(), size_of::<Self>());

        ((Self::Dunes as u8)..=(Self::GreyStone as u8)).map(|i| Self::try_from_primitive(i).unwrap())
    }

    #[inline]
    pub fn as_usize(self) -> usize {
        self as usize
    }

    pub fn sprite(self) -> Sprite {
        match self {
            Tile::Dunes => sprite!("dunes"),
            Tile::Mountains => sprite!("mountains"),
            Tile::Sand => sprite!("sand"),
            Tile::Snow => sprite!("snow"),
            Tile::Water => sprite!("water2"),
            Tile::Block => sprite!("block24"),
            Tile::Canal => sprite!("canal"),
            Tile::Farmland => sprite!("farmland"),
            Tile::Road => sprite!("road"),
            Tile::GreyStone => sprite!("greystone"),
        }
    }

    pub(crate) fn can_have_water(&self) -> bool {
        match self {
            Tile::Canal | Tile::Farmland => true,
            _ => false,
        }
    }

    /// 🥾 Can one generally walk on this kind of tile?
    pub fn is_default_walkable(self) -> bool {
        match self {
            Tile::Dunes => false,
            Tile::Mountains => false,
            Tile::Sand => true,
            Tile::Snow => true,
            Tile::Water => false,
            Tile::Block => false,
            Tile::Canal => false,
            Tile::Farmland => true,
            Tile::Road => true,
            Tile::GreyStone => false,
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Self as Debug>::fmt(self, f)
    }
}
