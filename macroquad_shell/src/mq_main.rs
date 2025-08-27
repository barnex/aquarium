use macroquad::prelude as mq;
use shell_api::*;

mod mq_resources;
use mq_resources::*;
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

    loop {
        res.poll(); // 👈 !

        mq::clear_background(mq::LIGHTGRAY);

        if let Some(texture) = res.get(&sprite!("kit6")) {
            mq::draw_texture(&texture, 0., 0., mq::WHITE);
        }

        mq::next_frame().await
    }
}
