use crate::prelude::*;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Button(pub Str16);

#[macro_export]
macro_rules! button {
    ($arg:tt) => {
        Button(str16!($arg))
    };
}

impl Button {
    pub const SCREENSHOT: Self = Self(Str16::from_slice(b"F12\0\0\0\0\0\0\0\0\0\0\0\0\0"));
    pub const MOUSE1: Self = Self(Str16::from_slice(b"Mouse1\0\0\0\0\0\0\0\0\0\0"));
    pub const MOUSE2: Self = Self(Str16::from_slice(b"Mouse2\0\0\0\0\0\0\0\0\0\0"));
    pub const FORWARD: Self = Self(Str16::from_slice(b"Forward\0\0\0\0\0\0\0\0\0"));
    pub const BACKWARD: Self = Self(Str16::from_slice(b"Backward\0\0\0\0\0\0\0\0"));
    pub const LEFT: Self = Self(Str16::from_slice(b"Left\0\0\0\0\0\0\0\0\0\0\0\0"));
    pub const RIGHT: Self = Self(Str16::from_slice(b"Right\0\0\0\0\0\0\0\0\0\0\0"));
    pub const CROUCH: Self = Self(Str16::from_slice(b"KeyZ\0\0\0\0\0\0\0\0\0\0\0\0"));
    pub const JUMP: Self = Self(Str16::from_slice(b"Space\0\0\0\0\0\0\0\0\0\0\0"));
    pub const ESC: Self = Self(Str16::from_slice(b"Escape\0\0\0\0\0\0\0\0\0\0"));
    pub const TAB: Self = Self(Str16::from_slice(b"Tab\0\0\0\0\0\0\0\0\0\0\0\0\0"));
    pub const SPACE: Self = Self(Str16::from_slice(b"Space\0\0\0\0\0\0\0\0\0\0\0"));
    pub const DIGIT1: Self = Self(Str16::from_slice(b"Digit1\0\0\0\0\0\0\0\0\0\0"));
    pub const DIGIT2: Self = Self(Str16::from_slice(b"Digit2\0\0\0\0\0\0\0\0\0\0"));
    pub const DIGIT3: Self = Self(Str16::from_slice(b"Digit3\0\0\0\0\0\0\0\0\0\0"));
    pub const DIGIT4: Self = Self(Str16::from_slice(b"Digit4\0\0\0\0\0\0\0\0\0\0"));
    pub const DIGIT5: Self = Self(Str16::from_slice(b"Digit5\0\0\0\0\0\0\0\0\0\0"));
    pub const DIGIT6: Self = Self(Str16::from_slice(b"Digit6\0\0\0\0\0\0\0\0\0\0"));
    pub const DIGIT7: Self = Self(Str16::from_slice(b"Digit7\0\0\0\0\0\0\0\0\0\0"));
    pub const DIGIT8: Self = Self(Str16::from_slice(b"Digit8\0\0\0\0\0\0\0\0\0\0"));
    pub const DIGIT9: Self = Self(Str16::from_slice(b"Digit9\0\0\0\0\0\0\0\0\0\0"));
    pub const DIGIT0: Self = Self(Str16::from_slice(b"Digit0\0\0\0\0\0\0\0\0\0\0"));
}
