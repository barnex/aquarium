use crate::prelude::*;

#[derive(Copy, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Mat {
    #[default]
    None = 0,
    Dunes = 1,
    Mountains = 2,
    Sand = 3,
    Snow = 4,
    Water = 5,
    Block = 6,
}

impl Mat {
    #[inline]
    pub fn as_usize(self) -> usize {
        self as usize
    }

    pub fn sprite(self) -> Sprite {
        match self {
            Mat::None => sprite!("grid24"),
            Mat::Dunes => sprite!("dunes"),
            Mat::Mountains => sprite!("mountains"),
            Mat::Sand => sprite!("sand"),
            Mat::Snow => sprite!("snow"),
            Mat::Water => sprite!("water2"),
            Mat::Block => sprite!("block24"),
        }
    }
}
