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
    pub viewport_size: vec2u,
    pub layers: Vec<Layer>,
    pub debug: String,
}

/// Command to draw a sprite.
#[derive(Debug, PartialEq, Eq)]
pub struct DrawSprite {
    pub sprite: Sprite,
    pub pos: vec2i,
    pub dst_size: Option<vec2<NonZeroU8>>,
}

impl DrawSprite {
    /// Draw sprite at position. Natural size.
    pub fn at_pos(sprite: Sprite, pos: vec2i) -> Self {
        Self { sprite, pos, dst_size: None }
    }

    pub fn with_size(self, dst_size: vec2u8) -> Self {
        if let (Some(x), Some(y)) = (NonZeroU8::new(dst_size.x()), NonZeroU8::new(dst_size.y())) {
            self.with(|s| s.dst_size = Some(vec2(x, y)))
        } else {
            debug_assert!(dst_size != vec2::ZERO, "zero dst_size");
            self
        }
    }
}

#[derive(Default, Debug, PartialEq, Eq)]
pub struct Layer {
    pub sprites: Vec<DrawSprite>,
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

    /// Draw sprite in world coordinates (i.e. taking into account camera).
    pub fn draw_sprite(&mut self, g: &G, layer: u8, sprite: Sprite, world_pos: vec2i) {
        self.layer(layer).sprites.push(DrawSprite::at_pos(sprite, world_pos - g.camera_pos));
    }

    /// Draw sprite in screen coordinates (i.e. ignoring camera).
    pub fn draw_sprite_screen(&mut self, layer: u8, sprite: Sprite, screen_pos: vec2i) {
        self.layer(layer).sprites.push(DrawSprite::at_pos(sprite, screen_pos));
    }

    pub fn push_sprite_with_size(&mut self, layer: u8, sprite: Sprite, pos: vec2i, dst_size: vec2u8) {
        self.layer(layer).sprites.push(DrawSprite::at_pos(sprite, pos).with_size(dst_size));
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
