use crate::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct Tilemap {
    size: vec2u,
    tiles: Vec<Cell<Tile>>,
}

impl Tilemap {
    pub fn new(size: vec2u) -> Self {
        let tiles = vec![Cell::new(Tile::Snow); (size.x() * size.y()).as_()];
        Self { size, tiles }
    }

    #[inline]
    pub fn size(&self) -> vec2u {
        self.size
    }

    #[inline]
    pub fn isize(&self) -> vec2i {
        self.size.as_i32()
    }

    #[inline]
    pub fn at(&self, idx: vec2i) -> Tile {
        match self.in_bounds(idx) {
            true => self.tiles[(idx.y() * self.isize().x() + idx.x()) as usize].get(),
            false => Tile::default(),
        }
    }

    #[inline]
    pub fn set(&self, idx: vec2i, v: Tile) {
        match self.in_bounds(idx) {
            true => self.tiles[(idx.y() * self.isize().x() + idx.x()) as usize].set(v),
            false => (),
        }
    }

    pub fn enumerate_all(&self) -> impl Iterator<Item = (vec2i, Tile)> {
        let (w, h) = self.isize().into();
        cross(0..w, 0..h).map(move |(x, y)| (vec2i(x, y), self.tiles[(y * w + x) as usize].get()))
    }

    #[inline]
    fn in_bounds(&self, idx: vec2i) -> bool {
        let (w, h) = self.size().as_i32().into();
        (0..w).contains(&idx.x()) && (0..h).contains(&idx.y())
    }
}
