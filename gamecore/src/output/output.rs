use crate::prelude::*;

/// Scenegraph, sounds, etc. to output after a tick.
/// Sent to the browser who will render it.
#[derive(Default, Debug, PartialEq, Eq)]
pub struct Output {
    pub sprites: Vec<(Sprite, vec2i)>,
    pub lines: Vec<Line>,

    pub debug: String,
}

impl Output {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn clear(&mut self) {
        self.sprites.clear();
        self.lines.clear();
        self.debug.clear();
        debug_assert!(self == &default());
    }
}
