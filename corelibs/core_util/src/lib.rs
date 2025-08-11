#![deny(warnings)]

//! Miscellaneous utilities and extension traits.

mod cartesian_product;
mod collect_grouped;
mod inspect_err;
mod replace_with;
mod saturating_add_assign;
mod with;
mod sorted;

pub use cartesian_product::*;
pub use collect_grouped::*;
pub use inspect_err::*;
pub use replace_with::*;
pub use saturating_add_assign::*;
pub use with::*;
pub use sorted::*;
