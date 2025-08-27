mod mq_resources;
use std::{
    collections::VecDeque,
    time::{self, Instant},
};

use mq_resources::*;
mod mq_storage;
use gamecore::*;
use mq_storage::*;

use macroquad::prelude as mq;
use shell_api::*;
use vector::*;

type HashMap<K, V> = fnv::FnvHashMap<K, V>;
type HashSet<T> = fnv::FnvHashSet<T>;

#[macroquad::main("Texture")]
async fn main() {
    env_logger::init();

    log::info!("Using macroquad shell");
    #[cfg(debug_assertions)]
    {
        log::warn!("debug_assertions enabled, performance will suffer");
    }

    let fallback = mq::Texture2D::from_image(&fallback_bitmap((0, 0, 255), vec2(24, 24) /*TODO*/));
    let mut res = Resources::new(fallback);

    let mut input_events = VecDeque::<InputEvent>::default();

    let mut state = match load_game() {
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

    loop {
        out.clear();
        res.poll(); // 👈 !

        out.viewport_size = vec2(mq::screen_width(), mq::screen_height()).as_u32();
        let now_secs = Instant::now().duration_since(start).as_secs_f64();
        state.tick(now_secs, input_events.drain(..), &mut out);

        println!("{ANSI_CLEAR}{}", &out.debug);

        mq::clear_background(mq::LIGHTGRAY);

        if let Some(texture) = res.get(&sprite!("kit6")) {
            mq::draw_texture(&texture, 0., 0., mq::WHITE);
        }

        mq::next_frame().await
    }
}

const ANSI_CLEAR: &'static str = "\x1B[2J\x1B[H";
