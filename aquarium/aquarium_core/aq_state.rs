use crate::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct AqState {
    pub now_secs: f64,
    pub tick: u64,

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
            (K_ARROW_DOWN, K_DOWN),
            (K_ARROW_LEFT, K_LEFT),
            (K_ARROW_RIGHT, K_RIGHT),
            (K_ARROW_UP, K_UP),
        ]);

        let console = Console::with_hotkey(K_CLI);

        let world = World::test();

        Self {
            now_secs: 0.0,
            tick: 0,
            keymap,
            inputs: default(),
            console,
            world,
        }
    }

    fn tick(&mut self, now_secs: f64, events: impl Iterator<Item = shell_api::InputEvent>, out: &mut shell_api::Out) {
        self.tick_bookkeeping(now_secs, events, out);

        self.tick_manual_control();

        self.world.tick();

        out.draw_text(0, (0, 0), "hello");
        self.world.draw(out);
    }

    fn tick_manual_control(&mut self) {
        let Some(crit0) = self.world.critters.get_mut(0) else { return };

        if self.inputs.is_down(K_LEFT) {
            crit0.head_pos += (-1.0, 0.0);
        }
        if self.inputs.is_down(K_RIGHT) {
            crit0.head_pos += (1.0, 0.0);
        }
        if self.inputs.is_down(K_DOWN) {
            crit0.head_pos += (0.0, 1.0);
        }
        if self.inputs.is_down(K_UP) {
            crit0.head_pos += (0.0, -1.0);
        }
    }

    fn tick_bookkeeping(&mut self, now_secs: f64, events: impl Iterator<Item = InputEvent>, out: &mut Out) {
        self.now_secs = now_secs;
        self.tick += 1;
        TICK_FOR_LOGGING.store(self.tick, std::sync::atomic::Ordering::Relaxed);

        self.inputs.tick(&self.keymap, events);
        self.console.tick_and_draw(&self.inputs, out).map(|cmd| self.exec_command(&cmd));

        for k in self.inputs.iter_just_pressed() {
            log::trace!("down: {k:?}");
        }
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
