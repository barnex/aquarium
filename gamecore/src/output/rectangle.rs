use crate::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Rectangle {
    pub bounds: Bounds,
    pub stroke: RGBA,
    pub fill: RGBA,
}

impl Rectangle {
    pub fn new(bounds: Bounds, stroke: RGBA) -> Self {
        Self { bounds, stroke, fill: RGBA::TRANSPARENT }
    }

    pub fn with_fill(self, fill: RGBA) -> Self {
        self.with(|v| v.fill = fill)
    }
}
