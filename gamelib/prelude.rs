pub use crate::*;

pub use cel::*;
pub use  vector::*;
pub use shell_api::*;
pub use geometry::*;
pub use core_util::*;

pub use serde::{Deserialize, Serialize};

pub fn toggle(v: &mut bool) {
    *v = !*v
}
