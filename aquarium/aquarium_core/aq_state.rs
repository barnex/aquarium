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

    pub contraptions: Vec<Contraption>,

    // commands and keypresses control this contraption.
    pub controlled_contraption: usize,

    // filter for smooth manual control
    mouse_filter: [vec2f; 3],

    pub crawl_amplitude: f32,
    pub crawl_wavenumber: f32,
    pub crawl_frequency: f32,
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

        let contraptions = vec![Contraption::rope(60), Contraption::rope(20)];

        Self {
            now_secs: 0.0,
            tick: 0,
            paused: false,
            keymap,
            inputs: default(),
            console,
            contraptions,
            mouse_filter: default(),
            controlled_contraption: 0,
            crawl_amplitude: default(),
            crawl_frequency: default(),
            crawl_wavenumber: default(),
        }
    }

    fn tick_and_draw(&mut self, now_secs: f64, events: impl Iterator<Item = shell_api::InputEvent>, out: &mut shell_api::Out) {
        self.update_inputs(now_secs, events);

        self.console.tick_and_draw(&self.inputs, out).map(|cmd| self.exec_command(&cmd));
        if !self.console.active {
            self.tick_manual_control();
        }

        if !self.paused || self.inputs.is_down(K_TICK) {
            self.tick_contraptions();
        }

        self.draw(out);
    }

    fn tick_contraptions(&mut self) {
        self.contraptions.iter_mut().for_each(|v| v.tick());
        self.tick_crawl_test();
    }

    fn tick_crawl_test(&mut self) {
        let Some(contraption) = self.contraptions.get_mut(self.controlled_contraption) else { return };

        //for (i,b)in contraption.sp
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

        self.mouse_filter[0] = self.inputs.mouse_position().as_();
        for i in 1..self.mouse_filter.len() {
            self.mouse_filter[i] = 0.7 * self.mouse_filter[i] + 0.3 * self.mouse_filter[i - 1];
        }

        let speed = 1.0;
        if let Some(c) = self.contraptions.get_mut(self.controlled_contraption) {
            if let Some(b) = c.bones.get_mut(0) {
                //b.body.position += speed * delta;
                //b.body.velocity = speed * delta;
                //b.body.position = self.inputs.mouse_position().as_();
                b.position = self.mouse_filter.last().copied().unwrap();
            }
        }
    }

    fn draw(&self, out: &mut Out) {
        draw_background(out);
        self.contraptions.iter().for_each(|v| v.draw(out));
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
            ["ctl", i] => Ok(self.controlled_contraption = i.parse()?),
            ["s", s] => Ok(self.controlled_contraption()?.stiffness = s.parse()?),
            ["n", n] => Ok(*self.controlled_contraption()? = Contraption::rope(n.parse()?)),
            ["g", g] => Ok(self.controlled_contraption()?.g = g.parse()?),
            ["k", k] => Ok({
                let k = k.parse()?;
                self.controlled_contraption()?.springs.iter_mut().for_each(|s| s.k = k)
            }),
            ["angle", a] => Ok({
                let a = f32::sin(a.parse()?);
                self.controlled_contraption()?.springs.iter_mut().for_each(|s| s.sin_angle = a)
            }),
            ["ca", v] => Ok(self.crawl_amplitude = v.parse::<f32>()?.clamp(-1.0, 1.0)),
            ["cf", v] => Ok(self.crawl_frequency = v.parse::<f32>()?.clamp(-1.0, 1.0)),
            ["cw", v] => Ok(self.crawl_wavenumber = v.parse::<f32>()?.clamp(-1.0, 1.0)),
            _ => Err(anyhow!("unknown command: {cmd:?}")),
        }
    }

    fn controlled_contraption(&mut self) -> Result<&mut Contraption> {
        self.contraptions.get_mut(self.controlled_contraption).ok_or_else(|| anyhow!("there is no contraption #{}", self.controlled_contraption))
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

fn not_found() -> Error {
    anyhow!("does not exist")
}
