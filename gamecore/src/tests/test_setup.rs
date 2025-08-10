use std::io::Write as _;
use std::sync::OnceLock;

use env_logger::fmt::style::{AnsiColor, Color};

use crate::prelude::*;

static LOGGER_INIT: OnceLock<()> = OnceLock::new();

pub(crate) fn init_logging() {
    LOGGER_INIT.get_or_init(|| {
        env_logger::builder()
            .is_test(true) // nicer formatting for `cargo test`
            .filter_level(log::LevelFilter::Trace)
            .write_style(env_logger::WriteStyle::Always)
            .format(|f, record| writeln!(f, "[{}] {}", record.level(), record.args()))
            .init();
    });
}

/// A small test world.
pub(crate) fn small_world() -> G {
    init_logging();
    let mut g = G::new(vec2(64, 32));

    for (x, y) in cross(2..5, 3..7) {
        g.tilemap.set(vec2(x, y), Tile::Water);
    }

    for (x, y) in cross(6..9, 2..5) {
        g.tilemap.set(vec2(x, y), Tile::Block);
    }

    g.tilemap.set(vec2(4, 4), Tile::Sand);

    g.viewport_size = vec2(480, 320);

    g
}
