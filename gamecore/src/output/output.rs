use crate::prelude::*;

/// The layer we draw the tilemap to.
pub const L_TILES: u8 = 0;

/// The layer we draw the sprites to.
pub const L_SPRITES: u8 = 1;

/// UI background layer (window fill etc).
pub const L_UI_BG: u8 = 2;

/// UI mid layer (text, buttons, ...).
pub const L_UI: u8 = 3;

/// UI foreground layer (selection markers etc).
pub const L_UI_FG: u8 = 4;

/// Scenegraph, sounds, etc. to output after a tick.
/// Sent to the browser who will render it.
#[derive(Default, Debug, PartialEq, Eq)]
pub struct Out {
    pub layers: Vec<Layer>,
    pub debug: String,
}

#[derive(Default, Debug, PartialEq, Eq)]
pub struct Layer {
    pub sprites: Vec<(Sprite, vec2i)>,
    pub lines: Vec<Line>,
    pub rectangles: Vec<Rectangle>,
}

impl Out {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn clear(&mut self) {
        self.layers.iter_mut().for_each(Layer::clear);
        self.debug.clear();
    }

    pub fn push_sprite(&mut self, layer: u8, sprite: Sprite, pos: vec2i) {
        self.layer(layer).sprites.push((sprite, pos));
    }

    pub fn push_line(&mut self, layer: u8, line: Line) {
        self.layer(layer).lines.push(line);
    }

    pub fn push_rect(&mut self, layer: u8, rect: Rectangle) {
        self.layer(layer).rectangles.push(rect);
    }

    fn layer(&mut self, layer: u8) -> &mut Layer {
        debug_assert!(layer <= 100, "too many layers");
        while self.layers.len() <= layer.as_() {
            self.layers.push(Layer::default());
        }
        &mut self.layers[layer as usize]
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
