use crate::prelude::*;

impl Tilemap {
    pub fn testmap(size: vec2u16) -> Self {
        Self::new(size).with(|m| {
            for (x, y) in cross(2..5, 3..7) {
                m.set(vec2(x, y), Tile::Water);
            }

            for (x, y) in cross(6..9, 2..5) {
                m.set(vec2(x, y), Tile::Block);
            }

            m.set(vec2(4, 4), Tile::Sand);
        })
    }
}
