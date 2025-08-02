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
    // WARNING: ‼️ keep in sync with `all`
}

impl Tile {
    // WARNING: ‼️ keep in sync with enum values.
    pub fn all() -> impl Iterator<Item = Self> {
        debug_assert_eq!(size_of::<Option<Self>>(), size_of::<Self>());

        ((Self::Dunes as u8)..=(Self::Block as u8)).map(|i| Self::try_from_primitive(i).unwrap())
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
        }
    }
}
