//! Utilities for setting up test game states.
use crate::prelude::*;
use crate::tests::test_utils::*;
use std::io::Write as _;
use std::path::PathBuf;
use std::sync::OnceLock;
use std::{env, fs};

static TEST_INIT: OnceLock<()> = OnceLock::new();

/// Initialize `env_logger` for testing.
/// Idempotent.
/// Called each time a test world is created.
fn init_test_logging() {
    TEST_INIT.get_or_init(|| {
        env_logger::builder()
            .is_test(true) // nicer formatting for `cargo test`
            .filter_level(log::LevelFilter::Trace)
            .write_style(env_logger::WriteStyle::Always)
            .format(|f, record| writeln!(f, "[{}] {}", record.level(), record.args()))
            .init();
        log::info!("wd: {:?}", env::current_dir());
    });
}

/// Test world with Headquarters, some resources and features.
pub(crate) fn world_with_resources(name: &str) -> G {
    let g = small_world(name);

    g.spawn_resource(vec2(11, 7), ResourceTyp::Rock);
    g.spawn_resource(vec2(11, 8), ResourceTyp::Rock);
    g.spawn_resource(vec2(11, 9), ResourceTyp::Rock);

    g
}

/// A small test world with some features.
pub(crate) fn small_world(name: &str) -> G {
    let mut g = test_world(vec2(64, 32), name);

    for (x, y) in cross(2..5, 3..7) {
        g.set_tile(vec2(x, y), Tile::Water);
    }
    for (x, y) in cross(6..9, 2..5) {
        g.set_tile(vec2(x, y), Tile::Block);
    }
    g.set_tile(vec2(4, 4), Tile::Sand);
    g
}

/// Base test world.
/// Test settings enabled. No features.
fn test_world(size: vec2u16, name: &str) -> G {
    init_test_logging();

    clean_output_dir(name);

    G::new(size, Team::Red).with(|g| {
        g.name = name.into(); // name used as output dir
        g.ui.hidden = true; // don't accidentally click on UI
        g.debug.draw_mouse = true; // see mouse position in screenshots
        g.debug.show_home = true;
        g.frames_per_tick = 1; // time moves fast, don't spend screenshots on pure animation
    })
}

/// Cleanup output from previous run.
fn clean_output_dir(name: &str) {
    let output_dir = test_output_dir(name);
    log::info!("rm {output_dir:?}");
    fs::remove_dir_all(output_dir).ignore_err();
}

/// Test Output directory for given `test_name!()`
pub(crate) fn test_output_dir(test_name: &str) -> PathBuf {
    PathBuf::from(format!("test_output/{test_name}/",))
}
