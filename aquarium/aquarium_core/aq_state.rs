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

    
    pub critters: Vec<Critter>,
    

    // commands and keypresses control this contraption.
    pub selected_critter: Option<usize>,
    pub follow_mouse: bool,

    // filter for smooth manual control
    mouse_filter: [vec2f; 3],

    pub dt: f32,
    pub speed: u32,
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

        let critters = vec![Critter::new(4)];

        Self {
            now_secs: 0.0,
            tick: 0,
            paused: false,
            keymap,
            inputs: default(),
            console,
            critters,
            mouse_filter: default(),
            selected_critter: Some(0),
            follow_mouse: false,
            dt: 0.02,
            speed: 1,
        }
    }

    fn tick_and_draw(&mut self, now_secs: f64, events: impl Iterator<Item = shell_api::InputEvent>, out: &mut shell_api::Out) {
        self.update_inputs(now_secs, events);

        self.console.tick_and_draw(&self.inputs, out).map(|cmd| self.exec_command(&cmd));
        if !self.console.active {
            self.tick_manual_control();
        }

        if !self.paused || self.inputs.is_down(K_TICK) {
            for _ in 0..self.speed {
                self.tick_contraptions();
            }
        }

        self.draw(out);
    }

    fn tick_contraptions(&mut self) {
        self.critters.iter_mut().for_each(|v| v.tick(self.now_secs, self.dt));
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

        if self.follow_mouse {
            if let Some(c) = self.selected_critter.and_then(|i| self.critters.get_mut(i)) {
                if let Some(b) = c.body.bones.get_mut(0) {
                    //b.body.position += speed * delta;
                    //b.body.velocity = speed * delta;
                    //b.body.position = self.inputs.mouse_position().as_();
                    b.position = self.mouse_filter.last().copied().unwrap();
                }
            }
        }
    }

    fn draw(&self, out: &mut Out) {
        draw_background(out);
        self.critters.iter().for_each(|v| v.draw(out));
        if let Some(sel) = self.selected_critter() {
            sel.brain.draw(out)
        }
        out.bloom = true;
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
            ["sel" | "select", i] => Ok(self.selected_critter = Some(i.parse()?)),
            ["s", s] => Ok(self.selected_critter_mut()?.body.stiffness = s.parse()?),
            ["n", n] => Ok(*self.selected_critter_mut()? = Critter::new(n.parse()?)),
            ["g", g] => Ok(self.selected_critter_mut()?.body.g = g.parse()?),
            ["k", k] => Ok({
                let k = k.parse()?;
                self.selected_critter_mut()?.body.springs.iter_mut().for_each(|s| s.k = k)
            }),
            ["angle", a] => Ok({
                let a = f32::sin(a.parse()?);
                self.selected_critter_mut()?.body.springs.iter_mut().for_each(|s| s.sin_angle = a)
            }),
            ["ca", v] => Ok(self.selected_critter_mut()?.crawl_amplitude = v.parse::<f32>()?),
            ["cf", v] => Ok(self.selected_critter_mut()?.crawl_frequency = v.parse::<f32>()?),
            ["cw", v] => Ok(self.selected_critter_mut()?.crawl_wavenumber = v.parse::<f32>()?),
            ["cg", v] => Ok(self.selected_critter_mut()?.crawl_gamma = v.parse::<f32>()?),
            ["mouse"] => Ok(toggle(&mut self.follow_mouse)),
            ["mouse", v] => Ok(self.follow_mouse = v.parse()?),
            ["dt", v] => Ok(self.dt = v.parse()?),
            ["speed", v] => Ok(self.speed = v.parse()?),
            _ => Err(anyhow!("unknown command: {cmd:?}")),
        }
    }

    fn selected_critter_mut(&mut self) -> Result<&mut Critter> {
        self.selected_critter.and_then(|i| self.critters.get_mut(i)).ok_or_else(|| anyhow!("there is no critter #{:?}", self.selected_critter))
    }

    fn selected_critter(&self) -> Option<&Critter> {
        self.selected_critter.and_then(|i| self.critters.get(i))
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

fn draw_background(out: &mut Out) {
    let (w, h) = out.viewport_size.as_i32().into();
    let bg = [0, 0, 80];
    out.draw_rect_screen(0, Rectangle::from((((0, 0), (w, h)), bg)).with_fill(bg));
}

fn not_found() -> Error {
    anyhow!("does not exist")
}
