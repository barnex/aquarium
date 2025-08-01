use crate::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct State {
    #[serde(skip)]
    pub keymap: Keymap,

    #[serde(skip)]
    pub inputs: Inputs,

    pub commands: VecDeque<String>,

    /// Screen/canvas size in pixels.
    pub viewport_size: vec2u,
    pub speed: u32,
    pub frame: u64,

    pub curr_time_secs: f64,
    pub dt: f64,
    pub dt_smooth: f64,
    pub score: u64, // 💀 remove

    pub camera_pos: vec2i,

    #[serde(skip)]
    pub ui: Ui,

    pub kits: Vec<(Sprite, vec2i, vec2i)>,
    pub tilemap: Tilemap,
}

pub const TILE_SIZE: u32 = 24;
pub const TILE_ISIZE: i32 = TILE_SIZE as i32;

impl State {
    pub fn new() -> Self {
        let sprites = [
            sprite!("kit0"),
            sprite!("kit1"),
            sprite!("kit2"),
            sprite!("kit3"),
            sprite!("kit4"),
            sprite!("kit5"),
            sprite!("kit6"),
            sprite!("kit7"),
            sprite!("kit8"),
            sprite!("kit9"),
            sprite!("blabla"), // 🪲 TODO: BUG: should load red replacement, but returns invalid bitmap or something
        ];

        let N = sprites.len();

        let mut rng = ChaCha8Rng::from_seed([42; 32]);

        let (w, h) = (480, 320);
        let kits = (0..8).map(|i| (sprites[i % N], vec2i(rng.gen_range(0..w), rng.gen_range(0..h)), vec2i(rng.gen_range(-3..=3), rng.gen_range(1..3)))).collect();

        Self {
            keymap: default_keybindings(),
            inputs: default(),

            commands: default(),
            tilemap: Tilemap::testmap(vec2(32, 24)),
            viewport_size: vec2(0, 0), // real value will be set by webshell.
            speed: 1,
            frame: 0,
            curr_time_secs: 0.0,
            dt: 1.0 / 60.0, // initial fps guess
            dt_smooth: 1.0 / 60.0,
            camera_pos: default(),
            ui: Ui::new(),
            score: default(),
            kits,
        }
    }

    pub fn tick(&mut self) {
        self.update_fps(); // 👈 FPS is gamespeed independent
        self.exec_commands(); // 👈 exec commands even when paused (speed 0)
        self.control_camera();
        self.doodle();

        for _ in 0..self.speed {
            self.tick_once();
        }
    }

    fn tick_once(&mut self) {
        self.frame += 1;

        self.bounce_kittens();
        self.do_something_on_keypress();
    }

    fn do_something_on_keypress(&mut self) {
        if self.inputs.just_pressed(Button(str16!("b"))) {
            self.score += 1
        }
    }

    fn doodle(&mut self) {
        if self.inputs.is_down(K_MOUSE1) {
            if let Some(mat) = self.ui.tile_picker {
                let idx = self.mouse_position_world() / TILE_ISIZE;
                self.tilemap.set(idx, mat);
            }
        }
    }

    fn mouse_position_world(&self) -> vec2i {
        self.inputs.mouse_position() + self.camera_pos
    }

    fn control_camera(&mut self) {
        let mut delta = vec2::ZERO;
        if self.inputs.is_down(K_CAM_DOWN) {
            delta += vec2(0, 1);
        }
        if self.inputs.is_down(K_CAM_UP) {
            delta += vec2(0, -1);
        }
        if self.inputs.is_down(K_CAM_LEFT) {
            delta += vec2(-1, 0);
        }
        if self.inputs.is_down(K_CAM_RIGHT) {
            delta += vec2(1, 0);
        }
        let speed = 3;
        self.camera_pos += speed * delta;
    }

    fn bounce_kittens(&mut self) {
        let size = [480, 320];
        for (_, pos, vel) in &mut self.kits {
            *pos += *vel;
            for i in 0..2 {
                if pos[i] > size[i] {
                    pos[i] = size[i];
                    vel[i] = -vel[i]
                }
                if pos[i] < 0 {
                    pos[i] = 0;
                    vel[i] = -vel[i]
                }
            }
        }
    }

    fn update_fps(&mut self) {
        self.dt = (self.inputs.now_secs - self.curr_time_secs).clamp(0.001, 0.1); // clamp dt to 1-100ms to avoid craziness on clock suspend etc.
        self.curr_time_secs = self.inputs.now_secs;
        self.dt_smooth = lerp(self.dt_smooth, self.dt, 0.02);
    }

    pub fn render(&mut self, out: &mut Output) {

        self.ui.update_and_draw(&mut self.inputs, out);

        debug_assert!(self.viewport_size != vec2::ZERO);
        self.draw_tilemap(out);

        for (kit, pos) in self.kits.iter().map(|(sprite, pos, _)| (*sprite, *pos - self.camera_pos)) {
            out.push_sprite(L_SPRITES, kit, pos);
        }
        out.push_sprite(L_SPRITES, sprite!("frame24"), self.inputs.mouse_position());

        self.output_debug(out);
    }

    fn output_debug(&self, out: &mut Output) {
        writeln!(&mut out.debug, "frame: {}, now: {:.04}s, FPS: {:.01}", self.frame, self.curr_time_secs, 1.0 / self.dt_smooth).unwrap();
        writeln!(&mut out.debug, "score {}", self.score).unwrap();
        writeln!(&mut out.debug, "camera {:?}", self.camera_pos).unwrap();
        writeln!(&mut out.debug, "viewport_size {:?}", self.viewport_size).unwrap();
        writeln!(&mut out.debug, "down {:?}", self.inputs.iter_is_down().sorted().collect_vec()).unwrap();
        writeln!(&mut out.debug, "tile_picker {:?}", self.ui.tile_picker).unwrap();
    }

    pub fn draw_tilemap(&self, out: &mut Output) {
        for (pos, mat) in self.tilemap.enumerate_all() {
            out.push_sprite(L_TILES, mat.sprite(), pos * TILE_ISIZE - self.camera_pos);
        }
    }
}
