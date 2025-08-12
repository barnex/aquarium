use crate::prelude::*;
use crate::tests::headless_renderer::render_headless;
use std::env;
use std::io::Write as _;
use std::path::PathBuf;
use std::sync::OnceLock;

static TEST_INIT: OnceLock<()> = OnceLock::new();

/// Initialize `env_logger` for testing.
/// Idempotent.
pub(crate) fn init_test_logging() {
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

/// Test Output directory for given `test_name!()`
pub(crate) fn test_output_dir(test_name: &str) -> PathBuf {
    PathBuf::from(format!("../test_output/{test_name}/",))
}

/// Render gamestate (headless), save under `test_output/<test_name>/frame_1234.png`.
pub fn screenshot(g: &mut G, out: &Out) {
    let fname = test_output_dir(&g.name).join(format!("frame_{:04}.png", g.frame));
    if let Some(dir) = fname.parent() {
        std::fs::create_dir_all(dir).log_err().swallow_err();
    }
    render_headless(&out, &fname).expect("save png");
    log::info!("wrote {fname:?}");
}

/// A small test world with some features.
pub(crate) fn small_world(name: &str) -> G {
    let g = test_world(vec2(64, 32), name);

    for (x, y) in cross(2..5, 3..7) {
        g.tilemap.set(vec2(x, y), Tile::Water);
    }
    for (x, y) in cross(6..9, 2..5) {
        g.tilemap.set(vec2(x, y), Tile::Block);
    }
    g.tilemap.set(vec2(4, 4), Tile::Sand);
    g
}

/// Base test world.
/// Test settings enabled. No features.
fn test_world(size: vec2u16, name: &str) -> G {
    init_test_logging();
    let mut g = G::new(size);
    g.name = name.into();
    g.ui.hidden = true;
    g
}
