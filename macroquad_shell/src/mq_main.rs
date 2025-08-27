mod mq_resources;
use mq_resources::*;
mod mq_storage;
use mq_storage::*;
use gamecore::*;


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


    loop {
        res.poll(); // 👈 !

        mq::clear_background(mq::LIGHTGRAY);

        if let Some(texture) = res.get(&sprite!("kit6")) {
            mq::draw_texture(&texture, 0., 0., mq::WHITE);
        }

        mq::next_frame().await
    }
}
