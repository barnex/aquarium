use crate::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct State {
    // 🌍 game world
    pub tilemap: Tilemap,
    pub buildings: Vec<Building>,

    // 🕣 timekeeping
    pub frame: u64,
    pub now_secs: f64,
    _prev_secs: f64,
    pub dt: f64,
    pub dt_smooth: f64,
    pub speed: u32,

    // 🕹️ input events
    #[serde(skip)]
    pub inputs: Inputs,
    pub commands: VecDeque<String>,
    #[serde(skip)]
    pub keymap: Keymap,

    // ⏯️ UI
    #[serde(skip)]
    pub ui: Ui,

    // 📺 output/rendering
    /// Screen/canvas size in pixels.
    pub viewport_size: vec2u,
    /// Camera position in world coordinates.
    pub camera_pos: vec2i,
    #[serde(skip)]
    pub out: Output,
}

pub const TILE_SIZE: u32 = 24;
pub const TILE_ISIZE: i32 = TILE_SIZE as i32;

impl State {
    pub fn new() -> Self {
        let buildings = vec![Building { typ: BuildingTyp::HQ, tile: vec2(12, 8) }];

        Self {
            buildings,
            camera_pos: default(),
            commands: default(),
            now_secs: 0.0,
            _prev_secs: 0.0,
            dt: 1.0 / 60.0, // initial fps guess
            dt_smooth: 1.0 / 60.0,
            frame: 0,
            inputs: default(),
            keymap: default_keybindings(),
            out: default(),
            speed: 1,
            tilemap: Tilemap::testmap(vec2(32, 24)),
            ui: Ui::new(),
            viewport_size: vec2(0, 0), // real value will be set by webshell.
        }
    }

    pub fn tick(&mut self) {
        self.out.clear();

        self.update_fps(); // 👈 FPS is gamespeed independent
        self.exec_commands(); // 👈 exec commands even when paused (speed 0)
        self.control_camera();

        self.ui.update_and_draw(&mut self.inputs, &mut self.out); // 👈 may consume inputs
        self.doodle();

        for _ in 0..self.speed {
            self.tick_once();
        }

        self.render_game();
    }

    pub fn render_game(&mut self) {
        debug_assert!(self.viewport_size != vec2::ZERO);

        // Note: ⚠️ UI already rendered (may consume input events)

        self.draw_tilemap();
        self.draw_buildings();
        self.draw_sprites();
        self.draw_cursor();
        self.output_debug();
    }

    pub fn draw_tilemap(&mut self) {
        for (pos, mat) in self.tilemap.enumerate_all() {
            self.out.push_sprite(L_TILES, mat.sprite(), pos * TILE_ISIZE - self.camera_pos);
        }
    }

    pub fn draw_buildings(&mut self) {
        for building in &self.buildings {
            self.out.push_sprite(L_SPRITES, building.typ.sprite(), building.tile * TILE_ISIZE - self.camera_pos);
        }
    }

    fn draw_cursor(&mut self) {
        let sprite = match self.ui.active_tool{
            Tool::Pointer => sprite!("grid24"),
            Tool::Tile(mat) => mat.sprite(),
        };
        self.out.push_sprite(L_SPRITES, sprite, self.mouse_tile() * TILE_ISIZE - self.camera_pos);
    }

    fn draw_sprites(&mut self) {
        //for (kit, pos) in self.kits.iter().map(|(sprite, pos, _)| (*sprite, *pos - self.camera_pos)) {
        //    self.out.push_sprite(L_SPRITES, kit, pos);
        //}
    }

    fn tick_once(&mut self) {
        self.frame += 1;
        //self.bounce_kittens();
    }

    fn doodle(&mut self) {
        if self.inputs.is_down(K_MOUSE1) {
            if let Tool::Tile(mat) = self.ui.active_tool {
                self.tilemap.set(self.mouse_tile(), mat);
            }
        }
    }

    fn mouse_position_world(&self) -> vec2i {
        self.inputs.mouse_position() + self.camera_pos
    }

    fn mouse_tile(&self) -> vec2i {
        self.mouse_position_world() / TILE_ISIZE
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

    fn update_fps(&mut self) {
        self.dt = (self.now_secs - self._prev_secs).clamp(0.001, 0.1); // clamp dt to 1-100ms to avoid craziness on clock suspend etc.
        self._prev_secs = self.now_secs;
        self.dt_smooth = lerp(self.dt_smooth, self.dt, 0.02);
    }

    fn output_debug(&mut self) {
        writeln!(&mut self.out.debug, "frame: {}, now: {:.04}s, FPS: {:.01}", self.frame, self.now_secs, 1.0 / self.dt_smooth).unwrap();
        writeln!(&mut self.out.debug, "camera {:?}", self.camera_pos).unwrap();
        writeln!(&mut self.out.debug, "viewport_size {:?}", self.viewport_size).unwrap();
        writeln!(&mut self.out.debug, "sprites {:?}", self.out.layers.iter().map(|l| l.sprites.len()).sum::<usize>()).unwrap();
        writeln!(&mut self.out.debug, "down {:?}", self.inputs.iter_is_down().sorted().collect_vec()).unwrap();
        writeln!(&mut self.out.debug, "tile_picker {:?}", self.ui.active_tool).unwrap();
    }
}
