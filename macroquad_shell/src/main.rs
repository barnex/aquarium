use macroquad::prelude::*;

#[macroquad::main("Texture")]
async fn main() {
    #[cfg(debug_assertions)]
    {
        log::warn!("debug_assertions enabled, performance will suffer");
    }

    let texture: Texture2D = load_texture("assets/ferris.png").await.unwrap();

    loop {
        clear_background(LIGHTGRAY);
        draw_texture(&texture, 0., 0., WHITE);
        next_frame().await
    }
}
