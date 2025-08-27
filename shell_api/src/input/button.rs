use crate::prelude::*;

/// Key code or mouse button. Backed by short string (`Str16`), copyable.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Button(pub Str16);

/// Constructor with compile-time size check (name fits `Str16`)
/// E.g. `button!("Mouse1")`.
#[macro_export]
macro_rules! button {
    ($arg:tt) => {
        Button(str16!($arg))
    };
}

impl Button {
}
