use crate::prelude::*;

pub const K_MOUSE1: Button = button!("Mouse1");
pub const K_MOUSE2: Button = button!("Mouse2");

pub const K_CAM_UP: Button = button!("cam_up");
pub const K_CAM_DOWN: Button = button!("cam_down");
pub const K_CAM_LEFT: Button = button!("cam_left");
pub const K_CAM_RIGHT: Button = button!("cam_right");

pub fn default_keymap() -> Keymap {
    Keymap(
        [
            (button!("s"), K_CAM_LEFT),  //_
            (button!("e"), K_CAM_UP),    //_
            (button!("d"), K_CAM_DOWN),  //_
            (button!("f"), K_CAM_RIGHT), //_
        ]
        .into_iter()
        .collect(),
    )
}
