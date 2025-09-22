mod mq_draw;
mod mq_inputs;
mod mq_resources;
mod mq_storage;
use mq_draw::*;
use mq_inputs::*;
use mq_resources::*;
use mq_storage::*;

use gamecore::*;

use std::collections::VecDeque;
use std::sync::atomic::AtomicU64;
use std::time::Instant;

use macroquad::prelude as mq;
use shell_api::*;
use vector::*;

type HashMap<K, V> = fnv::FnvHashMap<K, V>;

pub async fn lib_main() {
    init_logging();

    log::info!("Using macroquad shell");
    #[cfg(debug_assertions)]
    {
        log::warn!("macroquad shell: debug_assertions enabled, performance will suffer");
    }
    gamecore::init();

    let fallback = mq::Texture2D::from_image(&fallback_bitmap((0, 0, 255), vec2(24, 24) /*TODO*/));
    let mut res = Resources::new(fallback);
    let mut input_events = VecDeque::new();

    let mut g = match load_game() {
        Some(state) => {
            log::info!("game loaded");
            state
        }
        None => {
            log::info!("game not loaded, starting fresh");
            let mut g = G::test_world();
            #[cfg(debug_assertions)]
            {
                log::info!("enabling pause_on_sanity_failure (because debug_assertions)");
                g.debug.pause_on_sanity_failure = true;
            }
            g
        }
    };

    let mut out = Out::default();
    let start = Instant::now();

    mq::prevent_quit();
    loop {
        if mq::is_quit_requested() {
            log::info!("quitting...");
            // save_game(&g); <<< TODO
            return; // 👈 exit
        }

        let now_secs = Instant::now().duration_since(start).as_secs_f64();
        out.clear();

        out.viewport_size = vec2(mq::screen_width(), mq::screen_height()).as_u32();
        capture_input_events(&mut input_events);

        g.tick(now_secs, input_events.drain(..), &mut out);
        mq_draw(&mut res, &out);

        if !g.paused {
            //println!("{ANSI_CLEAR}{}", &out.debug);
            if !out.debug.is_empty() {
                println!("{}", &out.debug);
            }
        }
        if mq::is_key_pressed(mq::KeyCode::S) && mq::is_key_down(mq::KeyCode::LeftSuper) {
            save_game(&g);
        }
        if mq::is_key_pressed(mq::KeyCode::Space) {
            g.paused = true;
            g.commands.push_back("tick".into());
        }

        mq::next_frame().await
    }
}

fn init_logging() {
    use std::sync::atomic::Ordering::Relaxed;

    /// So that tracing can introduce a newline before each new tick that has logging.
    static LAST_TICK_WITH_TRACING_OUTPUT: AtomicU64 = AtomicU64::new(0);

    use env_logger::*;
    use log::*;
    use std::io::Write;
    env_logger::Builder::from_env(Env::default().default_filter_or("trace"))
        .format(|buf, record: &Record| {
            let file = record.file().unwrap_or("unknown");
            let line = record.line().map(|l| l.to_string()).unwrap_or("?".to_string());
            let tick = TICK_FOR_LOGGING.load(Relaxed);

            let maybe_newline = {
                // print a newline before the tracing output of each new tick.
                // but only once and only if that tick actually logs anything.
                let need_newline = tick != LAST_TICK_WITH_TRACING_OUTPUT.load(Relaxed);
                if need_newline {
                    LAST_TICK_WITH_TRACING_OUTPUT.store(tick, Relaxed);
                }
                if need_newline { "\n" } else { "" }
            };

            writeln!(buf, "{maybe_newline}[{:5} {tick:6} {file}:{line:4}] {}", record.level(), record.args())
        })
        .filter(None, LevelFilter::Trace)
        .init();
}
