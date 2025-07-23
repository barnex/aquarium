use crate::prelude::*;

/// Scenegraph, sounds, etc. to output after a tick.
#[derive(Default, Debug)]
pub struct Output {
    pub sprites: Vec<(Str16, vec2i)>,

    pub debug: String,
}

impl Output {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn clear(&mut self) {
        self.sprites.clear();
        self.debug.clear();
    }
}
