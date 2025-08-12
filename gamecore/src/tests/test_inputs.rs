//! Synthetic input events for driving tests.
use super::screenshot;
use crate::prelude::*;

/// Synthetically advance game state one tick, with given fake inputs happening right before the tick.
/// Time advances 16ms (~60 FPS)
pub fn tick(g: &mut G, inputs: impl IntoIterator<Item = InputEvent>) {
    // HACK to make input events relative to camera
    // Will not work when camera is moved in the same frame as mouse motion.
    let camera = g.camera_pos;
    let inputs = inputs
        .into_iter()
        .map(|event| match event {
            InputEvent::MouseMove { position } => InputEvent::MouseMove { position: position - camera },
            InputEvent::Key { button, direction } => InputEvent::Key { button, direction },
        })
        .collect_vec();

    let mut out = Out::default();
    out.viewport_size = vec2(480, 320);
    let now = g.now_secs + 0.016;
    g.tick(now, inputs.into_iter(), &mut out);

    screenshot(g, &out)

}

pub fn click_tile(tile: vec2i16) -> impl IntoIterator<Item = InputEvent> {
    [mouse_move_tile(tile), mouse_down(), mouse_up()]
}

pub fn mouse_move_tile(tile: impl Into<vec2i16>) -> InputEvent {
    let position = tile.into().pos();
    InputEvent::MouseMove { position }
}

pub fn mouse_down() -> InputEvent {
    InputEvent::Key { button: K_MOUSE1, direction: KeyDir::Down }
}

pub fn mouse_up() -> InputEvent {
    InputEvent::Key { button: K_MOUSE1, direction: KeyDir::Up }
}
