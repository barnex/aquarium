use crate::prelude::*;

/// Size (in pixels) of a single character in the embedded font map.
const _EMBEDDED_CHAR_SIZE: vec2u8 = vec2(8, 16);

/// Overall size (in pixels) of the embedded font map.
const EMBEDDED_FONTMAP_SIZE: vec2u = vec2u(128, 128);

/// Special symbol in font.png.
#[allow(unused)]
pub const FONT_HEART: &str = "\x02\x03";

/// Special symbol in font.png.
#[allow(unused)]
pub const FONT_PLUS: &str = "\x04\x05";

/// Special symbol in font.png.
#[allow(unused)]
pub const FONT_CROSSHAIR: &str = "\x06\x07";

/// Special symbol in font.png.
#[allow(unused)]
pub const FONT_SHIELD: &str = "\x08\x09";

/// A mesh for rendering text at a given position on the screen (using the embedded bitmap font).
/// Wraps long lines as shown below:
///
///   viewport size
///  +----------------+
///  |  `pos`+        |
///  |        your tex|
///  |        t here  |
///  |                |
///  +----------------+
///
pub fn layout_text(out: &mut Out, layer: u8, pos: vec2i, text: &str) {
    let viewport_size = out.viewport_size.as_i32();
    //let char_stride = (UI_SCALE as i32) * _EMBEDDED_CHAR_SIZE.as_i32();
    let char_stride = _EMBEDDED_CHAR_SIZE.as_i32();

    let mut char_pos = pos;
    for &char in text.as_bytes() {
        // newline
        if char == b'\n' {
            char_pos[0] = pos.x();
            char_pos[1] += char_stride.y();
            continue;
        }

        // wrap long lines
        if char_pos.x() > viewport_size.x() - char_stride.x() {
            char_pos[0] = pos.x();
            char_pos[1] += char_stride.y();
        }

        let src_pos = chr_tex_pos_16x8(char, _EMBEDDED_CHAR_SIZE);
        out.draw_sprite_screen_with_source(layer, sprite!("font"), src_pos, _EMBEDDED_CHAR_SIZE, char_pos);

        char_pos[0] += char_stride.x();
    }
}

/// Pixel position (top-left corner) of an ascii character in the embedded font map.
fn chr_tex_pos_16x8(char: u8, char_size: vec2u8) -> vec2u8 {
    let x = char & 0xf;
    let y = char >> 4;
    vec2(x, y) * char_size
}

fn text_height_chars(text: &str) -> u32 {
    text.lines().count() as u32
}

fn text_width_chars(text: &str) -> u32 {
    text.lines().map(str::len).max().unwrap_or(0) as u32
}

fn text_size_chars(text: &str) -> vec2u {
    vec2u(text_width_chars(text), text_height_chars(text))
}

fn text_size_pix(text: &str) -> vec2u {
    vec2u(text_width_chars(text), text_height_chars(text)) * (_EMBEDDED_CHAR_SIZE.as_u32())
}
