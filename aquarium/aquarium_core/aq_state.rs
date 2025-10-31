use crate::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct AqState {
    pub now_secs: f64,
    pub tick: u64,
    pub paused: bool,

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
            (K_ARROW_DOWN, K_DOWN),
            (K_ARROW_LEFT, K_LEFT),
            (K_ARROW_RIGHT, K_RIGHT),
            (K_ARROW_UP, K_UP),
            (K_SPACE, K_TICK),
        ]);

        let console = Console::with_hotkey(K_CLI);

        let world = World::test();

        Self {
            now_secs: 0.0,
            tick: 0,
            paused: false,
            keymap,
            inputs: default(),
            console,
            world,
        }
    }

    fn tick_and_draw(&mut self, now_secs: f64, events: impl Iterator<Item = shell_api::InputEvent>, out: &mut shell_api::Out) {
        self.update_inputs(now_secs, events);

        self.console.tick_and_draw(&self.inputs, out).map(|cmd| self.exec_command(&cmd));
        if !self.console.active {
            self.tick_manual_control();
        }

        if !self.paused || self.inputs.is_down(K_TICK) {
            self.world.tick();
        }

        self.draw(out);
    }

    fn tick_manual_control(&mut self) {
        let mut delta = vec2f::ZERO;
        if self.inputs.is_down(K_LEFT) {
            delta += (-1.0, 0.0);
        }
        if self.inputs.is_down(K_RIGHT) {
            delta += (1.0, 0.0);
        }
        if self.inputs.is_down(K_DOWN) {
            delta += (0.0, 1.0);
        }
        if self.inputs.is_down(K_UP) {
            delta += (0.0, -1.0);
        }

        if let Some(b) = self.world.bones.get_mut(0) {
            b.body.position += 0.5*delta;
        }
    }

    fn draw(&self, out: &mut Out) {
        self.world.draw(out);
    }

    fn update_inputs(&mut self, now_secs: f64, events: impl Iterator<Item = InputEvent>) {
        self.now_secs = now_secs;
        self.tick += 1;
        TICK_FOR_LOGGING.store(self.tick, std::sync::atomic::Ordering::Relaxed);

        self.inputs.tick(&self.keymap, events);

        for k in self.inputs.iter_just_pressed() {
            log::trace!("just pressed: {k:?}");
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
            ["pause"] => Ok(toggle(&mut self.paused)),
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
        self.tick_and_draw(now_secs, events, out)
    }

    fn reset(&mut self) {
        *self = Self::new()
    }
}

fn toggle(v: &mut bool) {
    *v = !*v
}
