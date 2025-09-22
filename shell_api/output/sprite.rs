use crate::prelude::*;

#[macro_export]
macro_rules! sprite {
    ($arg:tt) => {
        Sprite { file: proc_macros::str16!($arg) }
    };
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub struct Sprite {
    pub file: Str16,
}
