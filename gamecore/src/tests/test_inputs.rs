//! Synthetic input events for driving tests.

use crate::prelude::*;

pub fn tick(g: &mut G, inputs: impl IntoIterator<Item = InputEvent>) {
    // HACK to make input events relative to camera
    // Will not work when camera is moved in the same frame as mouse motion.
    let camera = g.camera_pos;
    let inputs = inputs.into_iter().map(|event|match event{
        InputEvent::MouseMove { position } => InputEvent::MouseMove { position: position - camera },
        InputEvent::Key { button, direction } => InputEvent::Key { button, direction }
    }).collect_vec();


    let mut out = Out::default();
    out.viewport_size = vec2(480, 320);
    let now = g.now_secs + 0.016;
    g.tick(now, inputs.into_iter(), &mut out);
}

pub fn click_tile(tile: vec2i16) -> impl IntoIterator<Item = InputEvent> {
    [mouse_move_tile(tile), mouse_down(), mouse_up()]
}

pub fn mouse_move_tile(tile: vec2i16) -> InputEvent {
    let position = tile.pos();
    InputEvent::MouseMove { position }
}

pub fn mouse_down() -> InputEvent {
    InputEvent::Key { button: K_MOUSE1, direction: KeyDir::Down }
}

pub fn mouse_up() -> InputEvent {
    InputEvent::Key { button: K_MOUSE1, direction: KeyDir::Up }
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
