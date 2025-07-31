use crate::prelude::*;

/// The layer we draw the tilemap to.
const L_TILES: u8 = 0;

/// The layer we draw the sprites to.
const L_SPRITES: u8 = 1;

/// UI background layer (window fill etc).
const L_UI_BG: u8 = 2;

/// UI mid layer (text, buttons, ...).
const L_UI: u8 = 3;

/// UI foreground layer (selection markers etc).
const L_UI_FG: u8 = 4;

/// Scenegraph, sounds, etc. to output after a tick.
/// Sent to the browser who will render it.
#[derive(Default, Debug, PartialEq, Eq)]
pub struct Output {
    pub curr_layer: u8,
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
        self.layers.iter_mut().for_each(Layer::clear);
        self.debug.clear();
    }

    pub fn push_sprite(&mut self, sprite: Sprite, pos: vec2i) {
        self.push_sprite_l(self.curr_layer, sprite, pos);
    }

    pub fn push_sprite_l(&mut self, layer: u8, sprite: Sprite, pos: vec2i) {
        self.layer(layer).sprites.push((sprite, pos));
    }

    pub fn push_line(&mut self, line: Line) {
        self.push_line_l(self.curr_layer, line);
    }

    pub fn push_line_l(&mut self, layer: u8, line: Line) {
        self.layer(layer).lines.push(line);
    }

    pub fn push_rect(&mut self, rect: Rectangle) {
        self.push_rect_l(self.curr_layer, rect);
    }

    pub fn push_rect_l(&mut self, layer: u8, rect: Rectangle) {
        self.layer(layer).rectangles.push(rect);
    }

    pub fn new_layer(&mut self) {
        self.curr_layer += 1
    }

    fn layer(&mut self, layer: u8) -> &mut Layer {
        debug_assert!(layer <= 100, "too many layers");
        while self.layers.len() <= layer.as_() {
            self.layers.push(Layer::default());
        }
        &mut self.layers[self.curr_layer as usize]
    }
}

impl Layer {
    fn clear(&mut self) {
        self.sprites.clear();
        self.lines.clear();
        self.rectangles.clear();
        debug_assert!(self == &Self::default(), "Layer::clear is correct");
    }
}
