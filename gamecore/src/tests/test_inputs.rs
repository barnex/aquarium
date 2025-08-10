//! Synthetic input events for driving tests.

use crate::prelude::*;

/// Synthetic click on a tile's position.
pub fn left_click_tile(g: &mut G, tile: vec2i16) {
    let world_pos_pixels = tile.pos();
    let screen_pos = world_pos_pixels - g.camera_pos;
    left_click_screen(g, screen_pos);
}

/// Synthetic click on pixel, screen coordinates.
pub fn left_click_screen(g: &mut G, screen_pos: vec2i) {
    log::trace!("click {screen_pos}");
    let keymap = Keymap::default();
    g.inputs.record_mouse_position(screen_pos);
    g.inputs.record_press(&keymap, K_MOUSE1);
    g.inputs.record_release(&keymap, K_MOUSE1);
}
