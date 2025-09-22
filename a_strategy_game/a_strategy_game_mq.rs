#[macroquad::main("Game")]
async fn main() {
    macroquad_shell::mq_main::<gamecore::G>().await
}
