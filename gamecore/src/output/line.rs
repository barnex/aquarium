use crate::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Line {
    pub start: vec2i,
    pub end: vec2i,
    pub color: RGB,
    pub width: u8,
}

impl Line {
    pub fn new(start: vec2i, end: vec2i) -> Self {
        Self { start, end, color: RGB::BLACK, width: 1 }
    }
}
