use crate::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct Tilemap {
    size: vec2u16,
    tiles: Vec<Cell<Tile>>,
}

impl Tilemap {
    pub fn new(size: vec2u16) -> Self {
        let tiles = vec![Cell::new(Tile::Snow); (size.x() * size.y()).as_()];
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
            true => self.tiles[(idx.y() * self.isize().x() + idx.x()) as usize].get(),
            false => Tile::default(),
        }
    }

    #[inline]
    pub fn set(&self, idx: vec2i16, v: Tile) {
        match self.in_bounds(idx) {
            true => self.tiles[(idx.y() * self.isize().x() + idx.x()) as usize].set(v),
            false => (),
        }
    }

    pub fn enumerate_all(&self) -> impl Iterator<Item = (vec2i16, Tile)> {
        let (w, h) = self.isize().into();
        cross(0..w, 0..h).map(move |(x, y)| (vec2i16(x, y), self.tiles[(y * w + x) as usize].get()))
    }

    #[inline]
    fn in_bounds(&self, idx: vec2i16) -> bool {
        let (w, h) = self.size().as_i16().into();
        (0..w).contains(&idx.x()) && (0..h).contains(&idx.y())
    }
}
