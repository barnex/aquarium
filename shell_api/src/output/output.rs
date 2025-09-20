use crate::prelude::*;

/// The layer we draw the tilemap to.
pub const L_TILES: u8 = 0;

pub const L_WATER: u8 = 1;

/// The layer we draw the sprites to.
pub const L_SPRITES: u8 = 2;

/// UI background layer (window fill etc).
pub const L_UI_BG: u8 = 3;

/// UI mid layer (text, buttons, ...).
pub const L_UI: u8 = 4;

/// UI foreground layer (selection markers etc).
pub const L_UI_FG: u8 = 5;

/// Text layer
pub const L_TEXT: u8 = 6;

/// Command-line interface
pub const L_CLI: u8 = 7;

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
    pub src_pos: Option<vec2u8>,
}

impl DrawSprite {
    /// Draw sprite at position. Natural size.
    pub fn at_pos(sprite: Sprite, pos: vec2i) -> Self {
        Self { sprite, pos, dst_size: None, src_pos: None }
    }

    pub fn with_size(self, dst_size: vec2u8) -> Self {
        if let (Some(x), Some(y)) = (NonZeroU8::new(dst_size.x()), NonZeroU8::new(dst_size.y())) {
            self.with(|s| s.dst_size = Some(vec2(x, y)))
        } else {
            debug_assert!(dst_size.x() != 0 && dst_size.y() != 0, "zero dst_size");
            self
        }
    }

    pub fn with_src_pos(self, src_pos: vec2u8) -> Self {
        self.with(|s| s.src_pos = Some(src_pos))
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

    /// Draw sprite in screen coordinates (i.e. ignoring camera).
    pub fn draw_sprite_screen(&mut self, layer: u8, sprite: Sprite, screen_pos: vec2i) {
        self.layer(layer).sprites.push(DrawSprite::at_pos(sprite, screen_pos));
    }

    pub fn draw_sprite_screen_with_size(&mut self, layer: u8, sprite: Sprite, pos: vec2i, dst_size: vec2u8) {
        self.layer(layer).sprites.push(DrawSprite::at_pos(sprite, pos).with_size(dst_size));
    }

    /// Draw a portion of sprite (E.g. sprite from atlas).
    /// +-------------------+
    /// |src_pos            |
    /// |     *----+        |
    /// |     |size|        |
    /// |     +----+        |
    /// |                   |
    /// +-------------------+
    pub fn draw_sprite_screen_with_source(&mut self, layer: u8, sprite: Sprite, src_pos: vec2u8, size: vec2u8, dst_pos: vec2i) {
        self.layer(layer).sprites.push(DrawSprite::at_pos(sprite, dst_pos).with_src_pos(src_pos).with_size(size));
    }

    pub fn draw_line_screen(&mut self, layer: u8, line: Line) {
        self.layer(layer).lines.push(line);
    }

    /// Draw rectangle in screen coordinates (i.e. ignoring camera).
    pub fn draw_rect_screen(&mut self, layer: u8, rect: Rectangle) {
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
