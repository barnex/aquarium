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

use test_utils::*;
use test_setup::*;

/// Name of current test function.
#[macro_export]
macro_rules! test_name {
    () => {{
        fn f() {}
        let name = std::any::type_name_of_val(&f);
        let name = name.strip_suffix("::f").expect("test_name");
        name.split("::").last().expect("test_name")
    }};
}
