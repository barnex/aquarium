use crate::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct Tilemap {
    size: vec2u16,
    tiles: Vec<Cell<Tile>>,
}

impl Tilemap {
    pub fn new(size: vec2u16) -> Self {
        // 👇 First `as usize`, then multiply to avoid overflow.
        let tiles = vec![Cell::new(Tile::Sand); size.x() as usize * size.y() as usize];
        Self { size, tiles }
    }

    #[inline]
    pub fn size(&self) -> vec2u16 {
        self.size
    }

    #[inline]
    pub fn isize(&self) -> vec2i16 {
        self.size.as_i16()
    }

    #[inline]
    pub fn at(&self, idx: vec2i16) -> Tile {
        match self.in_bounds(idx) {
            true => self.tiles[self.index(idx)].get(),
            false => Tile::default(),
        }
    }

    fn index(&self, idx: vec2i16) -> usize {
        idx.y() as usize * self.size().x() as usize + idx.x() as usize
    }

    #[inline]
    pub fn set(&self, idx: vec2i16, v: Tile) {
        match self.in_bounds(idx) {
            true => self.tiles[self.index(idx)].set(v),
            false => (),
        }
    }

    pub fn enumerate_all(&self) -> impl Iterator<Item = (vec2i16, Tile)> {
        let (w, h) = self.isize().into();
        cross(0..w, 0..h).map(move |(x, y)| (vec2i16(x, y), self.tiles[self.index(vec2(x,y))].get()))
    }

    #[inline]
    fn in_bounds(&self, idx: vec2i16) -> bool {
        let (w, h) = self.size().as_i16().into();
        (0..w).contains(&idx.x()) && (0..h).contains(&idx.y())
    }
}
