//! Testing approach.
//!
//! Ui interaction (clicking menus etc) is assumed to work -- any breakage will be immediately obvious.
//!
use crate::prelude::*;
use googletest::prelude::*;

mod headless_renderer;
mod test_utils;
mod test_setup;
mod ui_tests;
mod functional_tests;

use test_utils::*;
use test_setup::*;

