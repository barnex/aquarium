//! Testing approach.
//!
//! Ui interaction (clicking menus etc) is assumed to work -- any breakage will be immediately obvious.
//!
use crate::prelude::*;
use googletest::prelude::*;

mod functional_tests;
mod headless_renderer;
mod test_setup;
mod test_utils;
mod ui_tests;

use test_setup::*;
use test_utils::*;
