//! Utilities for driving test games forward:
//!   * Synthetic input events.
//!   * Synthetic `tick`s to advance time.
//!   * Save screenshots to disk.
use crate::prelude::*;
use crate::tests::headless_renderer::render_headless;
use crate::tests::test_setup::*;

/// Synthetically advance game state one tick, with given fake inputs happening right before the tick.
/// Time advances 16ms (~60 FPS)
pub fn tick(g: &mut G, inputs: impl IntoIterator<Item = InputEvent> + 'static) {
    fn tick(g: &mut G, inputs: Box<dyn Iterator<Item = InputEvent>>) {
        // HACK to make input events relative to camera
        // Will not work when camera is moved in the same frame as mouse motion.
        let camera = g.camera_pos;
        let inputs = inputs
            .into_iter()
            .inspect(|event| log::trace!("{event:?}"))
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

    tick(g, Box::new(inputs.into_iter()))
}

/// Tick `n` times.
pub fn tick_n(g: &mut G, n: usize) {
    for _ in 0..n {
        tick(g, []);
    }
}

/// Render gamestate (headless), save under `test_output/<test_name>/frame_1234.png`.
/// Automatically called on `tick`.
pub(crate) fn screenshot(g: &mut G, out: &Out) {
    let fname = test_output_dir(&g.name).join(format!("tick_{:04}.png", g.tick));
    if let Some(dir) = fname.parent() {
        std::fs::create_dir_all(dir).log_err().swallow_err();
    }
    render_headless(&out, &fname).expect("save png");
    log::info!("wrote {fname:?}");
}

/// InputEvents for clicking on a tile.
pub fn mouse_click_tile(tile: vec2i16) -> impl IntoIterator<Item = InputEvent> {
    [mouse_move_tile(tile), mouse_down(), mouse_up()]
}

/// Left click on current mouse position.
pub fn left_click() -> impl IntoIterator<Item = InputEvent> {
    [mouse_down(), mouse_up()]
}

/// Right click on current mouse position.
pub fn right_click() -> impl IntoIterator<Item = InputEvent> {
    [key_down(K_MOUSE2), key_up(K_MOUSE2)]
}

/// Move mouse to given tile coordinates.
pub fn mouse_move_tile(tile: impl Into<vec2i16>) -> InputEvent {
    let position = tile.into().pos();
    InputEvent::MouseMove { position }
}

/// Mouse button 1 down.
pub fn mouse_down() -> InputEvent {
    key_down(K_MOUSE1)
}

/// Mouse button 1 up.
pub fn mouse_up() -> InputEvent {
    key_up(K_MOUSE1)
}

/// Key or mouse button down.
pub fn key_down(button: Button) -> InputEvent {
    InputEvent::Key { button, direction: KeyDir::Down }
}

/// Key or mouse button up.
pub fn key_up(button: Button) -> InputEvent {
    InputEvent::Key { button, direction: KeyDir::Up }
}
