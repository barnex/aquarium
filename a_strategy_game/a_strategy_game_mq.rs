
#[macroquad::main("Game")]
async fn main() {
    macroquad_shell::lib_main::<gamecore::G>().await
}
