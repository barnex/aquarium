use crate::prelude::*;

pub const K_MOUSE1: Button = button!("Mouse1");
pub const K_MOUSE2: Button = button!("Mouse2");

pub const K_CAM_UP: Button = button!("cam_up");
pub const K_CAM_DOWN: Button = button!("cam_down");
pub const K_CAM_LEFT: Button = button!("cam_left");
pub const K_CAM_RIGHT: Button = button!("cam_right");

pub const K_CLI: Button = button!("cli");
pub const K_BACKSPACE: Button = button!("backspace");
pub const K_ESC: Button = button!("escape");
pub const K_ENTER: Button = button!("enter");

pub fn default_keybindings() -> Keymap {
    Keymap(
        [
            // Camera
            (button!("s"), K_CAM_LEFT),  //_
            (button!("e"), K_CAM_UP),    //_
            (button!("d"), K_CAM_DOWN),  //_
            (button!("f"), K_CAM_RIGHT), //_
            // Camera alt.
            (button!("ArrowLeft"), K_CAM_LEFT),   //_
            (button!("ArrowUp"), K_CAM_UP),       //_
            (button!("ArrowDown"), K_CAM_DOWN),   //_
            (button!("ArrowRight"), K_CAM_RIGHT), //_
            //
            (button!("tab"), K_CLI), // macroquad
            (button!("Tab"), K_CLI), // JS
        ]
        .into_iter()
        .collect(),
    )
}
