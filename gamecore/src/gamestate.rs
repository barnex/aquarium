use crate::prelude::*;

/// The Game State.
/// Short name `g: &G`, because it's passed down ehhhhhhverywhere.
#[derive(Serialize, Deserialize)]
pub struct G {
    // 🌍 game world
    pub name: String,
    pub tilemap: Tilemap,
    pub resources: ResourceMap,
    pub buildings: MemKeep<Building>,
    pub pawns: MemKeep<Pawn>,

    // ⏯️ UI
    #[serde(skip)]
    pub ui: Ui,

    /// What will happen when MOUSE2 is pressed. Depends on context.
    pub contextual_action: Action,

    /// Where selection rectangle started (mouse down position).
    pub selection_start: Option<vec2i>,

    /// Currently selected `Pawn`s.
    pub selected_pawn_ids: CSet<Id>,

    // 🕣 timekeeping
    pub paused: bool,
    pub frame: u64,
    pub tick: u64,
    pub now_secs: f64,
    _prev_secs: f64, // to compute dt
    pub dt: f64,
    pub dt_smooth: f64,
    pub frames_per_tick: u32,

    // 🕹️ input events
    #[serde(skip)]
    pub inputs: Inputs,
    pub commands: VecDeque<String>,
    #[serde(skip)]
    pub keymap: Keymap,

    // 📺 output/rendering
    /// Camera position in world coordinates.
    pub camera_pos: vec2i,

    /// Use methods `random_XYZ()`.
    pub(super) _rng: RefCell<ChaCha8Rng>,

    // 🪲 debug
    pub debug: DebugOpts,
    pub last_sanity_error: Option<String>,
}

pub const TILE_SIZE: u32 = 24;
pub const TILE_ISIZE: i32 = TILE_SIZE as i32;
pub const TILE_VSIZE: vec2i = vec2(TILE_ISIZE, TILE_ISIZE);

impl G {
    pub fn test_world() -> Self {
        let mut g = Self::new(vec2(48, 32));

        g.spawn(Pawn::new(PawnTyp::Kitten, vec2(17, 7)));
        let crab = g.spawn(Pawn::new(PawnTyp::Crablet, vec2(10, 4)).with(|p| p.cargo = Some(ResourceTyp::Leaf).cel()));
        let hq = g.spawn_building(Building::new(BuildingTyp::HQ, (12, 8))).unwrap();

        g.assign_to(crab, hq);

        g.spawn_resource((3, 9), ResourceTyp::Leaf);
        g.spawn_resource((7, 19), ResourceTyp::Rock);
        g.spawn_resource((17, 9), ResourceTyp::Rock);
        g.spawn_resource((15, 12), ResourceTyp::Leaf);

        #[cfg(debug_assertions)]
        {
            g.debug.show_home = true;
            g.debug.show_destination = true;
            g.debug.inspect_under_cursor = true;
        }

        g
    }

    pub fn new(size: vec2u16) -> Self {
        Self {
            name: "".into(),
            contextual_action: Action::None,
            resources: default(),
            selected_pawn_ids: default(),
            selection_start: None,
            buildings: MemKeep::new(),
            pawns: MemKeep::new(),
            camera_pos: vec2(40, 70), // nonzero so we notice offset issues without having to pan
            commands: default(),
            now_secs: 0.0,
            _prev_secs: 0.0,
            dt: 1.0 / 60.0, // initial fps guess
            dt_smooth: 1.0 / 60.0,
            frame: 0,
            tick: 0,
            paused: false,
            inputs: default(),
            keymap: default_keybindings(),
            frames_per_tick: 8,
            tilemap: Tilemap::new(size),
            ui: Ui::new(),
            _rng: RefCell::new(ChaCha8Rng::seed_from_u64(12345678)),
            debug: default(),
            last_sanity_error: None,
        }
    }

    /// ⏱️ Advance the game state one frame:
    ///   * Apply given input events and new wall time `now_secs`.
    ///   * Advance the state one tick.
    ///   * Render state to `out` (scenegraph).
    pub fn tick(&mut self, now_secs: f64, events: impl Iterator<Item = InputEvent>, out: &mut Out) {
        self.now_secs = now_secs;
        self.inputs.tick(&self.keymap, events);

        self.update_fps(); // 👈 FPS is gamespeed independent
        self.exec_commands(); // 👈 exec commands even when paused (speed 0)

        self.ui.update_and_draw(&mut self.inputs, out); // 👈 may consume inputs

        self.control();

        if !self.paused {
            self.frame += 1;
            if self.frame % (self.frames_per_tick as u64) == 0 {
                // 🪲 TODO: time major tick
                self.tick_once();
            }

            #[cfg(debug_assertions)]
            if self.debug.pause_on_sanity_failure {
                if let Err(e) = sanity_check(self) {
                    self.paused = true;
                    log::error!("sanity check failed, game paused: {e}");
                    self.last_sanity_error = Some(e.to_string());
                }
            }
        }

        self.draw_world(out);
        self.output_debug(&mut out.debug);

        self.pawns.gc();
        self.buildings.gc();
    }

    pub(crate) fn tick_once(&mut self) {
        self.tick += 1;
        self.tick_pawns();
    }

    pub(crate) fn tick_pawns(&mut self) {
        for p in self.pawns.iter() {
            p.tick(self);
        }
    }

    // -------------------------------- Tilemap

    /// Tile (e.g. Sand, Water, ...) at given index.
    pub fn tile_at(&self, idx: vec2i16) -> Tile {
        self.tilemap.at(idx)
    }

    /// 🥾 Can one generally walk on tile?
    /// TODO: ambiguous.
    pub(crate) fn is_walkable(&self, tile: vec2i16) -> bool {
        if !Self::tile_is_walkable(self.tilemap.at(tile)) {
            return false;
        }
        for b in self.buildings.iter() {
            if b.tile_bounds().contains(tile) && b.entrance() != tile {
                return false;
            }
        }
        true
    }

    /// 🧱 Can one generally build something on this tile?
    pub(crate) fn is_buildable(&self, tile: vec2i16) -> bool {
        if !Self::tile_is_walkable(self.tilemap.at(tile)) {
            return false;
        }
        for b in self.buildings.iter() {
            if b.tile_bounds().contains(tile) {
                return false;
            }
        }
        true
    }

    /// 🥾 Can one generally walk on this kind of tile?
    fn tile_is_walkable(tile: Tile) -> bool {
        match tile {
            Tile::Dunes => false,
            Tile::Mountains => false,
            Tile::Sand => true,
            Tile::Snow => true,
            Tile::Water => false,
            Tile::Block => false,
        }
    }

    // -------------------------------- Pawns

    /// Add a pawn to the game and return it (now with `id` set).
    pub fn spawn(&self, pawn: Pawn) -> &Pawn {
        log::trace!("spawn {:?} @ tile {}", pawn.typ, pawn.tile);
        self.pawns.insert(pawn)
    }

    /// Pawn with given Id, if any.
    pub fn pawn(&self, id: Id) -> Option<&Pawn> {
        self.pawns.get(id)
    }

    /// Iterate over all Pawns.
    pub fn pawns(&self) -> impl Iterator<Item = &Pawn> {
        self.pawns.iter()
    }

    /// Pawn at given position, if any.
    /// TODO: make faster via a hierarchy.
    pub fn pawn_at(&self, tile: vec2i16) -> Option<&Pawn> {
        self.pawns.iter().find(|v| v.tile == tile)
    }

    /// All currently selected Pawn Ids.
    pub fn selected_pawn_ids(&self) -> impl Iterator<Item = Id> {
        self.selected_pawn_ids.iter()
    }

    /// All currently selected Pawns.
    pub fn selected_pawns(&self) -> impl Iterator<Item = &Pawn> {
        self.selected_pawn_ids.iter().filter_map(|id| self.pawn(id))
    }

    /// Add pawn to selection.
    pub fn select_pawn(&self, id: Id) {
        self.selected_pawn_ids.insert(id);
    }

    // -------------------------------- Resources

    pub fn spawn_resource(&self, tile: impl Into<vec2i16>, resource: ResourceTyp) {
        self.resources.insert(tile.into(), resource);
    }

    // -------------------------------- Buildings

    /// Building with given Id, if any.
    pub fn building(&self, id: Id) -> Option<&Building> {
        self.buildings.get(id)
    }

    /// Iterate over all Buildings.
    pub fn buildings(&self) -> impl Iterator<Item = &Building> {
        self.buildings.iter()
    }

    /// Building at given position, if any.
    /// TODO: make faster via a hierarchy.
    pub fn building_at(&self, tile: vec2i16) -> Option<&Building> {
        self.buildings.iter().find(|v| v.tile_bounds().contains_incl(tile))
    }

    /// Add a building, if the location is suitable.
    pub fn spawn_building(&self, building: Building) -> Option<&Building> {
        let bounds = building.tile_bounds();
        let mut footprint = cross(bounds.x_range(), bounds.y_range());
        let can_build = footprint.all(|(x, y)| self.is_buildable(vec2(x, y)));
        if can_build { Some(self.buildings.insert(building)) } else { None }
    }

    /// 🏠 Assign pawn to work at building.
    pub fn assign_to(&self, pawn: &Pawn, building: &Building) {
        if let Some(home) = pawn.home(self) {
            home.workers.remove(&pawn.id);
        }
        building.workers.insert(pawn.id);
        pawn.home.set(Some(building.id));
    }

    // -------------------------------- Mouse

    /// Current mouse position in world coordinates.
    pub fn mouse_position_world(&self) -> vec2i {
        self.inputs.mouse_position() + self.camera_pos
    }

    /// Tile the mouse currently hovers over.
    pub fn mouse_tile(&self) -> vec2i16 {
        self.mouse_position_world().to_tile()
    }

    fn update_fps(&mut self) {
        self.dt = (self.now_secs - self._prev_secs).clamp(0.001, 0.1); // clamp dt to 1-100ms to avoid craziness on clock suspend etc.
        self._prev_secs = self.now_secs;
        self.dt_smooth = lerp(self.dt_smooth, self.dt, 0.02);
    }

    fn output_debug(&mut self, debug: &mut String) {
        if let Some(e) = self.last_sanity_error.as_ref() {
            writeln!(debug, "SANITY CHECK FAILED: {e}");
        }

        writeln!(debug, "now: {:.04}s, frame: {}, tick: {}, FPS: {:.01}", self.now_secs, self.frame, self.tick, 1.0 / self.dt_smooth).unwrap();
        writeln!(debug, "camera {:?}", self.camera_pos).unwrap();
        writeln!(debug, "down {:?}", self.inputs.iter_is_down().sorted().collect_vec()).unwrap();
        writeln!(debug, "tile_picker {:?}", self.ui.active_tool).unwrap();
        writeln!(debug, "selected: {:?}", self.selected_pawn_ids.len()).unwrap();
        writeln!(debug, "contextual_action: {:?}", self.contextual_action).unwrap();
    }
}
