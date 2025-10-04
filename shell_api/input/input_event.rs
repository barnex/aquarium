use crate::prelude::*;

#[derive(Debug)]
pub enum InputEvent {
    Key { button: Button, direction: KeyDir },
    MouseMove { position: vec2i },
    InputCharacter(char),
    Command(String),
}

#[derive(Debug)]
pub enum KeyDir {
    Down,
    Up,
}
