use crate::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Rectangle {
    pub bounds: Bounds,
    pub stroke: RGBA,
    pub fill: RGBA,
}

impl Rectangle {
    pub fn new(bounds: impl Into<Bounds>, stroke: impl Into<RGBA>) -> Self {
        Self {
            bounds: bounds.into(),
            stroke: stroke.into(),
            fill: RGBA::TRANSPARENT,
        }
    }

    /// Like `new`, but pass the top-left position and size.
    pub fn with_size(pos: impl Into<vec2i>, size: impl Into<vec2i>, stroke: impl Into<RGBA>) -> Self {
        Self::new(Bounds::with_size(pos.into(), size.into()), stroke)
    }

    /// Like `new`, but pass the center position and radius (half size).
    pub fn with_radius(pos: impl Into<vec2i>, size: impl Into<vec2i>, stroke: impl Into<RGBA>) -> Self {
        Self::new(Bounds::with_radius(pos.into(), size.into()), stroke)
    }

    pub fn with_fill(self, fill: impl Into<RGBA>) -> Self {
        self.with(|v| v.fill = fill.into())
    }

    pub fn translated(self, delta: Vector<i32, 2>) -> Self {
        self.with(|v| v.bounds = v.bounds.translated(delta))
    }
}

impl<B: Into<Bounds>, S: Into<RGBA>> From<(B, S)> for Rectangle {
    fn from((bounds, stroke): (B, S)) -> Self {
        Self::new(bounds.into(), stroke.into())
    }
}
