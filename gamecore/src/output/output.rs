use crate::prelude::*;

/// Scenegraph, sounds, etc. to output after a tick.
/// Sent to the browser who will render it.
#[derive(Default, Debug, PartialEq, Eq)]
pub struct Output {
    pub sprites: Vec<(Sprite, vec2i)>,
    pub lines: Vec<Line>,
    pub rectangles: Vec<Rectangle>,
    pub debug: String,
}

impl Output {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn clear(&mut self) {
        self.sprites.clear();
        self.lines.clear();
        self.rectangles.clear();
        self.debug.clear();
        debug_assert!(self == &default());
    }

    pub fn push_sprite(&mut self, sprite: Sprite, pos: vec2i) {
        self.sprites.push((sprite, pos));
    }

    pub fn push_line(&mut self, line: Line) {}

    pub fn push_rect(&mut self, rect: Rectangle) {
        self.rectangles.push(rect);
    }
}
