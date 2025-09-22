#[macroquad::main("Game")]
async fn main() {
    mq_libmain::lib_main::<gamecore::G>().await
}
