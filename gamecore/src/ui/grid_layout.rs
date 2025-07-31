use crate::prelude::*;

pub(super) struct GridLayout {
    bounds: Bounds2Di,
    cursor: vec2i,
}

impl GridLayout {
    pub fn new(bounds: Bounds2Di) -> Self {
        Self { bounds, cursor: bounds.min }
    }

    pub fn layout(&mut self, size: vec2u) -> vec2i {
        let size = size.as_i32();
        let result = self.cursor;

        self.cursor[0] += size.x();
        if self.cursor.x() >= self.bounds.max.x() {
            self.cursor[0] = self.bounds.min.x();
            self.cursor[1] += size.y();
        }

        result
    }
}
