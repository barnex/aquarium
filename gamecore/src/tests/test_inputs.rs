//! Synthetic input events for driving tests.

use crate::prelude::*;

pub fn tick(g: &mut G, inputs: impl IntoIterator<Item = InputEvent>) {
    let mut out = Out::default();
    g.tick(inputs, &mut out);
}

// /// Synthetic click on a tile's position.
// pub fn left_click_tile(g: &mut G, tile: vec2i16) {
//     left_click_screen(g, tile.pos() - g.camera_pos);
// }
// 
// /// Synthetic mouse down on a tile's position.
// pub fn left_mousedown_tile(g: &mut G, tile: vec2i16) {
//     left_mousedown_screen(g, tile.pos() - g.camera_pos);
// }
// 
// /// Synthetic mouse up on a tile's position.
// pub fn left_mouseup_tile(g: &mut G, tile: vec2i16) {
//     left_mouseup_screen(g, tile.pos() - g.camera_pos);
// }
// 
// pub fn mousemove_tile(g: &mut G, tile: vec2i16) {
//     mousemove_screen(g, tile.pos() - g.camera_pos);
// }
// 
// /// Synthetic click on pixel, screen coordinates.
// pub fn left_click_screen(g: &mut G, screen_pos: vec2i) {
//     log::trace!("click {screen_pos}");
//     let keymap = Keymap::default();
//     g.inputs.record_mouse_position(screen_pos);
//     g.inputs.record_press(&keymap, K_MOUSE1);
//     g.inputs.record_release(&keymap, K_MOUSE1);
// }
//
// /// Synthetic mouse down on pixel, screen coordinates.
// pub fn left_mousedown_screen(g: &mut G, screen_pos: vec2i) {
//     log::trace!("mousedown {screen_pos}");
//     let keymap = Keymap::default();
//     g.inputs.record_mouse_position(screen_pos);
//     g.inputs.record_press(&keymap, K_MOUSE1);
// }
//
// /// Synthetic mouse up on pixel, screen coordinates.
// pub fn left_mouseup_screen(g: &mut G, screen_pos: vec2i) {
//     log::trace!("mouseup {screen_pos}");
//     let keymap = Keymap::default();
//     g.inputs.record_mouse_position(screen_pos);
//     g.inputs.record_release(&keymap, K_MOUSE1);
// }
//
// pub fn mousemove_screen(g: &mut G, screen_pos: vec2i) {
//     log::trace!("mousemove {screen_pos}");
//     g.inputs.record_mouse_position(screen_pos);
// }
