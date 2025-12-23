pub use cel::*;
pub use core_util::*;
pub use geometry::*;
pub use shell_api::*;
pub use vector::*;

pub use serde::{Deserialize, Serialize};

pub fn toggle(v: &mut bool) {
    *v = !*v
}
