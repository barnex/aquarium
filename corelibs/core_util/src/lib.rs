#![deny(warnings)]

//! Miscellaneous utilities and extension traits.

mod cartesian_product;
mod collect_grouped;
mod ignore;
mod inspect_err;
mod replace_with;
mod saturating_add_assign;
mod sorted;
mod with;

pub use cartesian_product::*;
pub use collect_grouped::*;
pub use ignore::*;
pub use inspect_err::*;
pub use replace_with::*;
pub use saturating_add_assign::*;
pub use sorted::*;
pub use with::*;
