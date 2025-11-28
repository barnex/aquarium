use crate::prelude::*;

#[derive(Serialize, Deserialize, Clone)]
pub struct Vec2D<T> {
    pub size: vec2u,
    pub values: Vec<T>,
}

impl<T: Default> Vec2D<T> {
    pub fn new(size: vec2u) -> Self {
        let n = size.as_usize().product();
        // 👇 First `as usize`, then multiply to avoid overflow.
        let tiles = (0..n).map(|_| T::default()).collect();
        Self { size, values: tiles }
    }
}

impl<T> Vec2D<T> {
    #[inline]
    pub fn size(&self) -> vec2u {
        self.size
    }

    #[inline]
    pub fn isize(&self) -> vec2i {
        self.size.as_i32()
    }

    #[inline]
    pub fn at(&self, idx: vec2u) -> &T {
        &self.values[self.index(idx)]
    }

    #[inline]
    pub fn set(&mut self, idx: vec2u, v: T) {
        let i = self.index(idx);
        self.values[i] = v;
    }

    #[inline]
    pub fn at_mut(&mut self, idx: vec2u) -> &mut T {
        let i = self.index(idx);
        &mut self.values[i]
    }

    #[inline]
    fn index(&self, idx: vec2u) -> usize {
        idx.y() as usize * self.size().x() as usize + idx.x() as usize
    }

    #[inline]
    pub fn reverse_index(&self, i: usize) -> vec2u {
        let i = i as u32;
        let y = i / self.size().x();
        let x = i % self.size().x();
        vec2(x, y)
    }

    pub fn enumerate_ref(&self) -> impl Iterator<Item = (vec2u, &T)> {
        let (w, h) = self.size().into();
        cross(0..w, 0..h).map(move |(x, y)| (vec2(x, y), &self.values[self.index(vec2(x, y))]))
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.values.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.values.iter_mut()
    }

    #[inline]
    pub fn in_bounds(&self, idx: vec2i) -> bool {
        let (w, h) = self.size().as_i32().into();
        (0..w).contains(&idx.x()) && (0..h).contains(&idx.y())
    }
}

impl<T: Copy> Vec2D<T> {
    pub fn enumerate(&self) -> impl Iterator<Item = (vec2u, T)> {
        let (w, h) = self.size().into();
        cross(0..w, 0..h).map(move |(x, y)| (vec2(x, y), self.values[self.index(vec2(x, y))]))
    }
}
