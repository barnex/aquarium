use crate::prelude::*;

#[macro_export]
macro_rules! sprite {
    ($arg:tt) => {
        Sprite { file: str16!($arg) }
    };
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct Sprite {
    pub file: Str16,
}
