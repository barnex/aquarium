use crate::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct AqState {

}

impl AqState {
    fn tick(&mut self, now_secs: f64, events: impl Iterator<Item = shell_api::InputEvent>, out: &mut shell_api::Out) {
        out.draw_text(0, (0, 0), "hello");


    }
}

impl Default for AqState {
    fn default() -> Self {
        Self {}
    }
}

impl shell_api::GameCore for AqState {
    fn tick(&mut self, now_secs: f64, events: impl Iterator<Item = shell_api::InputEvent>, out: &mut shell_api::Out) {
        self.tick(now_secs, events, out)
    }

    fn push_command(&mut self, cmd: String) {}

    fn reset(&mut self) {}
}
