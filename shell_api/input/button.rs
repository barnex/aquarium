use crate::prelude::*;
use std::ops::Deref;

/// Key code or mouse button. Backed by short string (`Str16`), copyable.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Button(pub Str16);

pub const K_MOUSE1: Button = button!("Mouse1");
pub const K_MOUSE2: Button = button!("Mouse2");

pub const K_CLI: Button = button!("cli");
pub const K_BACKSPACE: Button = button!("backspace");
pub const K_ESC: Button = button!("escape");
pub const K_ENTER: Button = button!("enter");

pub const K_ARROW_LEFT: Button = button!("arrowleft");
pub const K_ARROW_UP: Button = button!("arrowup");
pub const K_ARROW_DOWN: Button = button!("arrowdown");
pub const K_ARROW_RIGHT: Button = button!("arrowright");

pub const K_LEFT: Button = button!("left");
pub const K_UP: Button = button!("up");
pub const K_DOWN: Button = button!("down");
pub const K_RIGHT: Button = button!("right");

pub const K_SPACE: Button = button!("space");
pub const K_TICK: Button = button!("tick");

/// Constructor with compile-time size check (name fits `Str16`)
/// E.g. `button!("Mouse1")`.
#[macro_export]
macro_rules! button {
    ($arg:tt) => {
        Button(str16!($arg))
    };
}

impl Deref for Button {
    type Target = Str16;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
