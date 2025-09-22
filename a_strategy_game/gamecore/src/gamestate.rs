use crate::prelude::*;
use std::sync::atomic::AtomicU64;

/// Only so that trace logging can print the current tick.
/// ⚠️ For anything else, use G::tick.
pub static TICK_FOR_LOGGING: AtomicU64 = AtomicU64::new(0);

/// The Game State.
/// Short name `g: &G`, because it's passed down ehhhhhhverywhere.
#[derive(Serialize, Deserialize)]
pub struct G {
    // 🌍 game world
    pub name: String,
    pub _tilemap: Tilemap,
    pub resources: ResourceMap,
    pub buildings: MemKeep<Building>,
    pub pawns: MemKeep<Pawn>,
    pub water: WaterSim,
    pub header_text: String,

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
    pub viewport_size: vec2u,

    /// Use methods `random_XYZ()`.
    pub(super) _rng: RefCell<ChaCha8Rng>,

    // 🪲 debug
    pub debug: DebugOpts,
    pub last_sanity_error: Option<String>,

    pub console: Console,
}

pub const TILE_SIZE: u32 = 24;
pub const TILE_ISIZE: i32 = TILE_SIZE as i32;
pub const TILE_VSIZE: vec2i = vec2(TILE_ISIZE, TILE_ISIZE);

impl G {
    pub fn test_world() -> Self {
        map_gen::inception()
    }

    pub fn new(size: vec2u16) -> Self {
        let mut debug = DebugOpts::default();
        #[cfg(debug_assertions)]
        {
            //debug.show_home = true;
            //debug.show_destination = true;
            //debug.inspect_under_cursor = true;
        }

        Self {
            _prev_secs: 0.0,
            _rng: RefCell::new(ChaCha8Rng::seed_from_u64(12345678)),
            _tilemap: Tilemap::new(size),
            buildings: MemKeep::new(),
            camera_pos: vec2(40, 70), // nonzero so we notice offset issues without having to pan
            commands: default(),
            contextual_action: Action::None,
            debug,
            dt: 1.0 / 60.0, // initial fps guess
            dt_smooth: 1.0 / 60.0,
            frame: 0,
            frames_per_tick: 8,
            header_text: default(),
            inputs: default(),
            keymap: default_keybindings(),
            last_sanity_error: None,
            name: "".into(),
            now_secs: 0.0,
            paused: false,
            pawns: MemKeep::new(),
            resources: default(),
            selected_pawn_ids: default(),
            selection_start: None,
            tick: 0,
            ui: Ui::new(),
            viewport_size: vec2(0, 0),
            water: default(),
            console: default(),
        }
    }

    /// ⏱️ Advance the game state one frame:
    ///   * Apply given input events and new wall time `now_secs`.
    ///   * Advance the state one tick.
    ///   * Render state to `out` (scenegraph).
    pub fn tick(&mut self, now_secs: f64, events: impl Iterator<Item = InputEvent>, out: &mut Out) {
        self.now_secs = now_secs;
        self.inputs.tick(&self.keymap, events);
        self.viewport_size = out.viewport_size;

        self.update_fps(); // 👈 FPS is gamespeed independent
        self.exec_commands(); // 👈 exec commands even when paused (speed 0)

        self.tick_console();
        // 👇 📺 console overlays normal game. Disables game control when active.
        if !self.console.active {
            self.ui.update_and_draw(&mut self.inputs, out); // 👈 may consume inputs
            self.control();
        }

        if !self.paused {
            self.frame += 1;

            if self.frame % (self.frames_per_tick as u64) == 0 {
                // 🪲 TODO: time major tick
                self.major_tick();
                self.water.major_tick(&self._tilemap);
            } else {
                // 🪲 TODO: properly pace, make testable
                self.water.minor_tick(&self._tilemap);
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
        self.draw_console(out);

        //write_debug_output(self, out);
        //debug_println!(3, "hi");

        self.pawns.gc();
        self.buildings.gc();
    }

    pub(crate) fn major_tick(&mut self) {
        self.tick += 1;
        TICK_FOR_LOGGING.store(self.tick, std::sync::atomic::Ordering::Relaxed);
        self.tick_pawns();
        self.tick_farmland();

        // tick text
        {
            self.header_text.clear();
            write!(&mut self.header_text, "{}", self.name).swallow_err();

            // print total resources
            let mut total_resources = [0u32; ResourceTyp::COUNT];
            for b in self.buildings.iter() {
                for (res, count) in b.iter_resources() {
                    total_resources[res as usize] += count as u32
                }
            }
            for res in ResourceTyp::all() {
                let count = total_resources[res as usize];
                write!(&mut self.header_text, " | {res:?}:{count}").swallow_err();
            }
        }
    }

    pub(crate) fn tick_pawns(&mut self) {
        for p in self.pawns.iter() {
            p.tick(self);
        }
    }

    pub(crate) fn tick_farmland(&mut self) {
        //let growth_rate = 0.01;
        //for tile in farmland_tiles {
        //    if self.water_level_at(tile) > 0.01 {
        //        if self.resources.at(tile).is_none() && self.random::<f32>() < growth_rate * self.water_level_at(tile) {
        //            self.spawn_resource(tile, ResourceTyp::Leaf);
        //            *self.water.h.get_mut(&tile).unwrap() = 0.0;
        //        }
        //    }
        //}
        let mut buf = Vec::new();
        for (&pos, water) in self.water.farm_water.iter_mut() {
            if *water >= 100.0 {
                // Some randomization so all crops don't appear simultaneously.
                *water = self._rng.borrow_mut().gen_range(0.0..20.0);
                buf.push(pos);
            }
        }
        for pos in buf {
            self.spawn_resource(pos, ResourceTyp::Leaf);
        }
    }

    // -------------------------------- Water

    pub fn water_level_at(&self, tile: vec2i16) -> f32 {
        self.water.water_level_at(tile)
    }

    // -------------------------------- Tilemap

    /// Tile (e.g. Sand, Water, ...) at given index.
    pub fn tile_at(&self, idx: vec2i16) -> Tile {
        self._tilemap.at(idx)
    }

    /// 🥾 Can one generally walk on tile?
    /// TODO: ambiguous.
    pub(crate) fn is_walkable(&self, tile: vec2i16) -> bool {
        if !Self::tile_is_walkable(self.tile_at(tile)) {
            return false;
        }
        // 🪲 TODO: very inefficient
        //for b in self.buildings.iter() {
        //    if b.tile_bounds().contains(tile) && b.entrance() != tile {
        //        return false;
        //    }
        //}
        true
    }

    /// 🧱 Can one generally build something on this tile?
    pub(crate) fn is_buildable(&self, tile: vec2i16) -> bool {
        if !Self::tile_is_walkable(self.tile_at(tile)) {
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
            Tile::Canal => false,
            Tile::Farmland => true,
            Tile::Road => true,
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
        if !can_build {
            return None;
        }
        let building = self.buildings.insert(building);
        self.update_downstream_buildings();
        Some(building)
    }

    fn update_downstream_buildings(&self) {
        let Some(hq) = self.buildings().find(|b| b.typ == BuildingTyp::HQ) else { return log::error!("No HQ") };

        // 🪲 TODO: quadratic in #buildings. Use spatial queries instead.
        const MAX_DIST2: i32 = 30 * 30; // TODO
        for building in self.buildings().filter(|b| b.id != hq.id) {
            let my_resources = building.iter_resources().map(|(r, _)| r).collect::<HashSet<_>>();
            let neighbors = self
                .buildings() //_
                .filter(|b| b.id != building.id)
                .filter(|b| b.is_depot())
                .filter(|b| b.tile.distance_squared(building.tile) < MAX_DIST2)
                .filter(|b| b.tile.distance_squared(hq.tile) < building.tile.distance_squared(hq.tile))
                .filter(|b| b.iter_resources().map(|(r, _)| r).any(|r| my_resources.contains(&r)))
                .map(|b| b.id);
            building._downstream.clear();
            building._downstream.extend(neighbors);
        }

        // upstream
        // for building in self.buildings() {
        //     let my_resources = building.iter_resources().map(|(r, _)| r).collect::<HashSet<_>>();
        //     let neighbors = self
        //         .buildings() //_
        //         .filter(|b| b.id != building.id)
        //         .filter(|b| b.tile.distance_squared(building.tile) < MAX_DIST2)
        //         .filter(|b| b.tile.distance_squared(hq.tile) >= building.tile.distance_squared(hq.tile))
        //         .filter(|b| b.iter_resources().map(|(r, _)| r).any(|r| my_resources.contains(&r)))
        //         .map(|b| b.id);
        //     building._downstream.clear();
        //     building._downstream.extend(neighbors);
        // }
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

    pub fn set_tile(&mut self, idx: Vector<i16, 2>, v: Tile) {
        self._tilemap.set(idx, v);

        // 💧 add 0 water to canal, to kickstart water sim
        if v == Tile::Canal {
            self.water.h.entry(idx).or_default();
        }

        if v != Tile::Canal {
            self.water.h.remove(&idx);
            self.water.p.remove(&idx);
        }

        if v != Tile::Farmland {
            self.water.farm_water.remove(&idx);
        }

        // ☘️ resource becomes unreachable, remove it.
        if !self.is_walkable(idx) {
            self.resources.remove(idx);
        }
    }
}


impl GameCore for G{
    fn tick(&mut self, now_secs: f64, events: impl Iterator<Item = InputEvent>, out: &mut Out) {
        self.tick(now_secs, events, out)
    }
    
    fn tick_for_logging() -> u64 {
        TICK_FOR_LOGGING.load(std::sync::atomic::Ordering::Relaxed)
    }
    
}

impl Default for G{
    fn default() -> Self {
        Self::test_world()
    }
}