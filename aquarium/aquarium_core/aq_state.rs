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

    pub contraptions: Vec<Assembly>,

    // filter for smooth manual control
    manual_ctl: [vec2f; 3],
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

        let contraptions = vec![Assembly::rope(60), Assembly::rope(20)];

        Self {
            now_secs: 0.0,
            tick: 0,
            paused: false,
            keymap,
            inputs: default(),
            console,
            contraptions,
            manual_ctl: default(),
        }
    }

    fn tick_and_draw(&mut self, now_secs: f64, events: impl Iterator<Item = shell_api::InputEvent>, out: &mut shell_api::Out) {
        self.update_inputs(now_secs, events);

        self.console.tick_and_draw(&self.inputs, out).map(|cmd| self.exec_command(&cmd));
        if !self.console.active {
            self.tick_manual_control();
        }

        if !self.paused || self.inputs.is_down(K_TICK) {
            self.contraptions.iter_mut().for_each(|v| v.tick());
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

        self.manual_ctl[0] = self.inputs.mouse_position().as_();
        for i in 1..self.manual_ctl.len() {
            self.manual_ctl[i] = 0.7 * self.manual_ctl[i] + 0.3 * self.manual_ctl[i - 1];
        }

        let speed = 1.0;
        if let Some(c) = self.contraptions.get_mut(0) {
            if let Some(b) = c.bones.get_mut(0) {
                //b.body.position += speed * delta;
                //b.body.velocity = speed * delta;
                //b.body.position = self.inputs.mouse_position().as_();
                b.position = self.manual_ctl.last().copied().unwrap();
            }
        }
    }

    fn draw(&self, out: &mut Out) {
        draw_background(out);
        self.contraptions.iter().for_each(|v|v.draw(out));
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
            ["reset"] => Ok(self.reset()),
            //["n", n] => Ok(self.world = Assembly::rope(n.parse()?)),
            //["g", g] => Ok(self.world.g = g.parse()?),
            //["k", k] => Ok({
            //    let k = k.parse()?;
            //    self.world.springs.iter_mut().for_each(|s| s.k = k)
            //}),
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

fn draw_body(out: &mut Out, body: &RigidBody) {
    let pos = body.position.as_i32();
    let color = RGBA::WHITE;

    // draw center
    let s = vec2(2, 2);
    out.draw_rect_screen(L_SPRITES, Rectangle::new((pos - s, pos + s), color));

    // draw frame/axes
    let ax_len = 15.0;
    let x = body.transform_rel_pos(vec2::EX * ax_len).as_i32();
    let y = body.transform_rel_pos(vec2::EY * ax_len).as_i32();
    out.draw_line_screen(L_SPRITES, Line::new(pos, x).with_color(color));
    out.draw_line_screen(L_SPRITES, Line::new(pos, y).with_color(color));
}

fn draw_background(out: &mut Out) {
    let (w, h) = out.viewport_size.as_i32().into();
    let bg = [0, 0, 80];
    out.draw_rect_screen(0, Rectangle::from((((0, 0), (w, h)), bg)).with_fill(bg));
}
