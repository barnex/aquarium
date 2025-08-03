use std::ops::{Deref, DerefMut};

use crate::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct State {
    // 🌍 game world
    pub tilemap: Tilemap,
    pub buildings: Vec<Building>,
    pub pawns: MemKeep<Pawn>,

    // ⏯️ UI
    #[serde(skip)]
    pub ui: Ui,
    pub selected: Cel<Option<Id>>,

    // 🕣 timekeeping
    pub frame: u64,
    pub now_secs: f64,
    _prev_secs: f64, // to compute dt
    pub dt: f64,
    pub dt_smooth: f64,
    pub speed: u32,

    // 🕹️ input events
    #[serde(skip)]
    pub inputs: Inputs,
    pub commands: VecDeque<String>,
    #[serde(skip)]
    pub keymap: Keymap,

    // 📺 output/rendering
    /// Screen/canvas size in pixels.
    pub viewport_size: vec2u,
    /// Camera position in world coordinates.
    pub camera_pos: vec2i,
}

pub const TILE_SIZE: u32 = 24;
pub const TILE_ISIZE: i32 = TILE_SIZE as i32;

impl State {
    pub fn new() -> Self {
        let buildings = vec![Building { typ: BuildingTyp::HQ, tile: vec2(12, 8) }];
        let pawns = MemKeep::new();
        pawns.insert(Pawn::new(PawnTyp::Leaf, vec2(17, 7)));

        Self {
            selected: None.cel(),
            buildings,
            pawns,
            camera_pos: vec2(40, 70), // nonzero so we notice offset issues without having to pan
            commands: default(),
            now_secs: 0.0,
            _prev_secs: 0.0,
            dt: 1.0 / 60.0, // initial fps guess
            dt_smooth: 1.0 / 60.0,
            frame: 0,
            inputs: default(),
            keymap: default_keybindings(),
            speed: 1,
            tilemap: Tilemap::testmap(vec2(64, 48)),
            ui: Ui::new(),
            viewport_size: vec2(0, 0), // real value will be set by webshell.
        }
    }

    pub fn tick(&mut self, out: &mut Output) {
        self.update_fps(); // 👈 FPS is gamespeed independent
        self.exec_commands(); // 👈 exec commands even when paused (speed 0)

        self.ui.update_and_draw(&mut self.inputs, out); // 👈 may consume inputs

        self.control();

        for _ in 0..self.speed {
            self.tick_once();
        }

        self.draw_world(out);
        self.output_debug(&mut out.debug);
    }

    pub(crate) fn tick_once(&mut self) {
        self.frame += 1;
    }
    
    pub fn pawn(&self, id: Id) -> Option<&Pawn> {
        self.pawns.get(id)
    }

    pub fn pawn_at(&self, tile: vec2i16) -> Option<&Pawn> {
        self.pawns.iter().find(|v| v.tile == tile)
    }

    pub fn mouse_position_world(&self) -> vec2i {
        self.inputs.mouse_position() + self.camera_pos
    }

    pub fn mouse_tile(&self) -> vec2i16 {
        self.mouse_position_world().to_tile()
    }

    fn update_fps(&mut self) {
        self.dt = (self.now_secs - self._prev_secs).clamp(0.001, 0.1); // clamp dt to 1-100ms to avoid craziness on clock suspend etc.
        self._prev_secs = self.now_secs;
        self.dt_smooth = lerp(self.dt_smooth, self.dt, 0.02);
    }

    fn output_debug(&mut self, debug: &mut String) {
        writeln!(debug, "frame: {}, now: {:.04}s, FPS: {:.01}", self.frame, self.now_secs, 1.0 / self.dt_smooth).unwrap();
        writeln!(debug, "camera {:?}", self.camera_pos).unwrap();
        writeln!(debug, "viewport_size {:?}", self.viewport_size).unwrap();
        //writelnt.debug, "buildings: {}, pawns: {}", self.buildings.len(), self.pawns.len()).unwrap();
        //writeln!(debug, "sprites {:?}", self.out.layers.iter().map(|l| l.sprites.len()).sum::<usize>()).unwrap();
        writeln!(debug, "down {:?}", self.inputs.iter_is_down().sorted().collect_vec()).unwrap();
        writeln!(debug, "tile_picker {:?}", self.ui.active_tool).unwrap();
        writeln!(debug, "selected: {:?}: {:?}", self.selected, self.selected.map(|id|self.pawn(id))).unwrap();
    }
}
