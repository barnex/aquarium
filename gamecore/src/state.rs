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
        self.control_camera();

        self.ui.update_and_draw(&mut self.inputs, out); // 👈 may consume inputs
        //
        self.doodle();

        select_pawns(self);
        command_pawns(self);

        for _ in 0..self.speed {
            self.tick_once();
        }

        self.render_game(out);
    }

    pub fn render_game(&mut self, out: &mut Output) {
        debug_assert!(self.viewport_size != vec2::ZERO);

        // Note: ⚠️ UI already rendered (may consume input events)

        self.draw_tilemap(out);
        self.draw_buildings(out);
        self.draw_pawns(out);
        self.draw_selection(out);
        self.draw_cursor(out);
        self.output_debug(&mut out.debug);
    }

    pub fn draw_tilemap(&mut self, out: &mut Output) {
        for (idx, mat) in self.tilemap.enumerate_all() {
            out.push_sprite(L_TILES, mat.sprite(), idx.pos() - self.camera_pos);
        }
    }

    pub fn draw_buildings(&mut self, out: &mut Output) {
        for building in &self.buildings {
            out.push_sprite(L_SPRITES, building.typ.sprite(), building.tile * TILE_ISIZE - self.camera_pos);
        }
    }

    fn draw_pawns(&mut self, out: &mut Output) {
        for pawn in self.pawns.iter() {
            out.push_sprite(L_SPRITES, pawn.typ.sprite(), pawn.tile.pos() - self.camera_pos);
        }
    }

    fn draw_cursor(&mut self, out: &mut Output) {
        let sprite = match self.ui.active_tool {
            Tool::Pointer => sprite!("grid24"),
            Tool::Tile(typ) => typ.sprite(),
            Tool::Pawn(typ) => typ.sprite(),
        };
        out.push_sprite(L_SPRITES, sprite, self.mouse_tile().pos() - self.camera_pos);
        out.push_sprite(L_SPRITES, sprite!("grid24"), self.mouse_tile().pos() - self.camera_pos);
    }

    fn draw_selection(&mut self, out: &mut Output) -> Option<()> {
        let sel = self.pawns.get(self.selected.get()?)?;

        out.push_rect(L_SPRITES, Rectangle::new(sel.bounds().translated(-self.camera_pos), RGBA::BLUE).with_fill(RGB::BLUE.with_alpha(64)));

        OK
    }

    pub(crate) fn tick_once(&mut self) {
        self.frame += 1;
    }

    fn doodle(&mut self) {
        if self.inputs.is_down(K_MOUSE1) {
            if let Tool::Tile(mat) = self.ui.active_tool {}

            match self.ui.active_tool {
                Tool::Pointer => (),
                Tool::Tile(mat) => self.tilemap.set(self.mouse_tile(), mat),
                Tool::Pawn(typ) => {
                    if self.inputs.just_pressed(K_MOUSE1) {
                        self.pawns.insert(Pawn::new(typ, self.mouse_tile()));
                    }
                }
            }
        }
    }

    fn pawn_at(&self, tile: vec2i16) -> Option<&Pawn> {
        self.pawns.iter().find(|v| v.tile == tile)
    }

    fn mouse_position_world(&self) -> vec2i {
        self.inputs.mouse_position() + self.camera_pos
    }

    fn mouse_tile(&self) -> vec2i16 {
        self.mouse_position_world().to_tile()
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

    fn output_debug(&mut self, debug: &mut String) {
        writeln!(debug, "frame: {}, now: {:.04}s, FPS: {:.01}", self.frame, self.now_secs, 1.0 / self.dt_smooth).unwrap();
        writeln!(debug, "camera {:?}", self.camera_pos).unwrap();
        writeln!(debug, "viewport_size {:?}", self.viewport_size).unwrap();
        //writelnt.debug, "buildings: {}, pawns: {}", self.buildings.len(), self.pawns.len()).unwrap();
        //writeln!(debug, "sprites {:?}", self.out.layers.iter().map(|l| l.sprites.len()).sum::<usize>()).unwrap();
        writeln!(debug, "down {:?}", self.inputs.iter_is_down().sorted().collect_vec()).unwrap();
        writeln!(debug, "tile_picker {:?}", self.ui.active_tool).unwrap();
        writeln!(debug, "selected: {:?}", self.selected).unwrap();
    }
}

fn select_pawns(s: &mut State) {
    if s.ui.active_tool == Tool::Pointer {
        if s.inputs.just_pressed(K_MOUSE1) {
            if let Some(pawn) = s.pawn_at(s.mouse_tile()) {
                s.selected.set(Some(pawn.id))
            }
        }
    }
}

fn command_pawns(s: &mut State) {
    if s.ui.active_tool == Tool::Pointer {
        if s.inputs.just_pressed(K_MOUSE1) {}

        if let Some(pawn) = s.pawn_at(s.mouse_tile()) {
            log::info!("{pawn:?}");
        }
    }
}
