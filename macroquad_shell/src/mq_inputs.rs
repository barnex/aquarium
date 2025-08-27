use std::str::FromStr as _;

use crate::*;
use fixed_str::*;

pub fn capture_input_events(dst: &mut VecDeque<InputEvent>) {
    for code in mq::get_keys_pressed() {
        if let Ok(button) = Str16::from_str(&format!("{code:?}").to_ascii_lowercase()) {
            dst.push_back(InputEvent::Key { button: Button(button), direction: KeyDir::Down });
        }
    }
    for code in mq::get_keys_released() {
        if let Ok(button) = Str16::from_str(&format!("{code:?}").to_ascii_lowercase()) {
            dst.push_back(InputEvent::Key { button: Button(button), direction: KeyDir::Up });
        }
    }
    for (mq_button, button) in [(mq::MouseButton::Left, K_MOUSE1), (mq::MouseButton::Right, K_MOUSE2)] {
        if mq::is_mouse_button_pressed(mq_button) {
            dst.push_back(InputEvent::Key { button, direction: KeyDir::Down });
        }
        if mq::is_mouse_button_released(mq_button) {
            dst.push_back(InputEvent::Key { button, direction: KeyDir::Up });
        }
    }

    dst.push_back(InputEvent::MouseMove {
        position: vec2::from(mq::mouse_position()).as_i32(),
    });
}
