/// Handy conversion between tiles (vec2i16) and world positions (vec2i).
use crate::prelude::*;

pub(crate) trait ToTile {
    /// Truncate world position (vec2i) to tile (vec2i16).
    fn to_tile(self) -> vec2i16;
}

pub(crate) trait ToPos {
    /// Convert tile (vec2i16) to world position (vec2i).
    fn pos(self) -> vec2i;
}

impl ToTile for vec2i {
    #[inline]
    fn to_tile(self) -> vec2i16 {
        (self / TILE_ISIZE).as_i16()
    }
}

impl ToTile for &Cel<vec2i> {
    #[inline]
    fn to_tile(self) -> vec2i16 {
        self.get().to_tile()
    }
}

impl ToPos for vec2i16 {
    #[inline]
    fn pos(self) -> vec2i {
        self.as_i32() * TILE_ISIZE
    }
}

impl ToPos for &Cel<vec2i16> {
    #[inline]
    fn pos(self) -> vec2i {
        self.get().pos()
    }
}
