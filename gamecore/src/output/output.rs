use crate::prelude::*;

/// Scenegraph, sounds, etc. to output after a tick.
/// Sent to the browser who will render it.
#[derive(Default, Debug, PartialEq, Eq)]
pub struct Output {
    pub curr_layer: usize,
    pub layers: Vec<Layer>,
    pub debug: String,
}

#[derive(Default, Debug, PartialEq, Eq)]
pub struct Layer {
    pub sprites: Vec<(Sprite, vec2i)>,
    pub lines: Vec<Line>,
    pub rectangles: Vec<Rectangle>,
}

impl Output {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn clear(&mut self) {
        self.curr_layer = 0;
        self.layers.clear();
        self.debug.clear();
        debug_assert!(self == &default());
    }

    pub fn push_sprite(&mut self, sprite: Sprite, pos: vec2i) {
        self.curr_layer().sprites.push((sprite, pos));
    }

    pub fn push_line(&mut self, line: Line) {
        self.curr_layer().lines.push(line);
    }

    pub fn push_rect(&mut self, rect: Rectangle) {
        self.curr_layer().rectangles.push(rect);
    }

    pub fn new_layer(&mut self) {
        self.curr_layer += 1
    }

    fn curr_layer(&mut self) -> &mut Layer {
        while self.layers.len() <= self.curr_layer {
            self.layers.push(Layer::default());
        }
        &mut self.layers[self.curr_layer]
    }
}
