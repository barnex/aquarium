use crate::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct AqState {
    pub keymap: Keymap,

    #[serde(skip)]
    pub inputs: Inputs,

    pub console: Console,

    pub world: World,
}

impl AqState {
    pub fn new() -> Self {
        let keymap = Keymap::from([
            (button!("tab"), K_CLI), // macroquad
            (button!("Tab"), K_CLI), // JS
        ]);

        let console = Console::with_hotkey(K_CLI);

        let world = World::test();

        Self { keymap, inputs: default(), console, world }
    }

    fn tick(&mut self, now_secs: f64, events: impl Iterator<Item = shell_api::InputEvent>, out: &mut shell_api::Out) {
        self.inputs.tick(&self.keymap, events);
        self.console.tick_and_draw(&self.inputs, out).map(|cmd| self.exec_command(&cmd));

        self.world.tick();

        out.draw_text(0, (0, 0), "hello");
        self.world.draw(out);
    }

    fn exec_command(&mut self, cmd: &str) {
        match self.exec_command_impl(cmd) {
            Ok(_) => self.console.print("ok"),
            Err(e) => self.console.print(format!("{e}")),
        }
    }
    fn exec_command_impl(&mut self, cmd: &str) -> Result<()> {
        match cmd.trim().split_ascii_whitespace().collect_vec().as_slice() {
            _ => Err(anyhow!("unknown command: {cmd:?}")),
        }
    }
}

impl Default for AqState {
    fn default() -> Self {
        Self::new()
    }
}

impl shell_api::GameCore for AqState {
    fn tick(&mut self, now_secs: f64, events: impl Iterator<Item = shell_api::InputEvent>, out: &mut shell_api::Out) {
        self.tick(now_secs, events, out)
    }

    fn reset(&mut self) {}
}
