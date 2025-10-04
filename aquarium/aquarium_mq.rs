#[macroquad::main("Game")]
async fn main() {
    macroquad_shell::mq_main::<aquarium_core::AqState>().await
}
