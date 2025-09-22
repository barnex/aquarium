use crate::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Line {
    pub start: vec2i,
    pub end: vec2i,
    pub color: RGBA,
    pub width: u8,
}

impl Line {
    pub fn new(start: vec2i, end: vec2i) -> Self {
        Self { start, end, color: RGBA::BLACK, width: 1 }
    }

    #[must_use = "does not modify original"]
    pub fn translated(self, delta: vec2i) -> Self {
        self.with(|v| {
            v.start += delta;
            v.end += delta
        })
    }

    pub fn with_color(self, color: RGBA) -> Self {
        self.with(|v| v.color = color)
    }

    pub fn with_width(self, width: u8) -> Self {
        self.with(|v| v.width = width)
    }
}
