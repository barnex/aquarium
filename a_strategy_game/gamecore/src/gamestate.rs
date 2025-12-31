#[allow(mismatched_lifetime_syntaxes)]
use crate::prelude::*;

/// The Game State.
/// Short name `g: &G`, because it's passed down ehhhhhhverywhere.
#[derive(Serialize, Deserialize)]
pub struct G {
    // 🕣 timekeeping
    pub frame: u64,
    pub now_micros: u64,

    // 📺 Rendering FPS estimate
    prev_frame_micros: u64,
    dt: f64,
    dt_smooth: f64,

    // 🌍 Simulation pacing
    pub paused: bool,
    pub curr_sim_tick: u64,
    pub target_secs_per_sim_tick: f64,
    //pub last_tick_instant: f64,

    // 🌍 game world
    pub name: String,
    pub _tilemap: Tilemap,
    pub resources: ResourceMap,
    pub renewables: Renewables,

    pub entities: Entities,

    pub water: WaterSim,
    pub header_text: String,

    #[serde(skip)]
    pub effects: Effects,

    // ⏯️ UI
    pub player: Team,
    #[serde(skip)]
    pub ui: GameUi,
    /// What will happen when MOUSE2 is pressed. Depends on context.
    pub contextual_action: Action,
    /// Where selection rectangle started (mouse down position).
    pub selection_start: Option<vec2i>,
    /// Currently selected `Pawn`s.
    pub selected_entity_ids: CSet<Id>,
    pub inspected: CSet<Id>,

    // 🕹️ input events
    #[serde(skip)]
    pub inputs: Inputs,
    pub commands: VecDeque<String>,
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
    // ________________________________________________________________________________ entities

    //pub fn try_spawn<T: EntityT + HasTypeId>(&self, e: T) -> Option<&T> {
    //    if e.can_spawn(self) {
    //        Some(self.spawn(e))
    //    } else {
    //        trace!(e, "cannot spawn here");
    //        None
    //    }
    //}

    pub fn spawn<T: EntityT + HasTypeId>(&self, e: T) -> &T {
        debug_assert_eq!(e.id(), Id::INVALID, "need fresh entity");
        self.entities.insert(e).with(|e| e.on_spawned(self))
    }

    pub fn kill<'g>(&'g self, e: impl Into<Entity<'g>>) {
        let e = e.into();
        self.entities.remove(e.id());
        e.on_killed(self)
    }

    pub fn entity(&self, id: Id) -> Option<Entity> {
        self.entities.get_dyn(id)
    }

    pub fn get<T>(&self, id: Id) -> Option<&T>
    where
        T: EntityT + HasTypeId,
    {
        self.entities.get(id)
    }

    pub fn entities(&self) -> impl Iterator<Item = Entity> {
        self.entities.iter_dyn()
    }

    // ________________________________________________________________________________
    pub fn test_world() -> Self {
        let g = map_gen::inception();
        g
    }

    pub fn new(size: vec2u16, player: Team) -> Self {
        let mut debug = DebugOpts::default();
        #[cfg(debug_assertions)]
        {
            //debug.show_home = true;
            //debug.show_destination = true;
            //debug.inspect_under_cursor = true;
            debug.pause_on_sanity_failure = true;
        }

        let keymap = Keymap::from([
            // Camera
            (button!("s"), K_CAM_LEFT),  //_
            (button!("e"), K_CAM_UP),    //_
            (button!("d"), K_CAM_DOWN),  //_
            (button!("f"), K_CAM_RIGHT), //_
            // Camera alt.
            (button!("arrowleft"), K_CAM_LEFT),   //_
            (button!("arrowup"), K_CAM_UP),       //_
            (button!("arrowdown"), K_CAM_DOWN),   //_
            (button!("arrowright"), K_CAM_RIGHT), //_
            //
            (button!("tab"), K_CLI), // macroquad
        ]);

        Self {
            prev_frame_micros: 0,
            _rng: RefCell::new(ChaCha8Rng::seed_from_u64(12345678)),
            _tilemap: Tilemap::new(size),
            player,
            camera_pos: vec2(40, 70), // nonzero so we notice offset issues without having to pan
            commands: default(),
            contextual_action: Action::None,
            debug,
            target_secs_per_sim_tick: 0.200,
            dt: 1.0 / 60.0, // initial fps guess
            dt_smooth: 1.0 / 60.0,
            frame: 0,
            header_text: default(),
            inputs: default(),
            keymap,
            last_sanity_error: None,
            name: "".into(),
            now_micros: 0,
            paused: false,
            entities: Entities::new(),
            resources: default(),
            renewables: default(),
            selected_entity_ids: default(), // <<< TODO: remove
            inspected: default(),
            selection_start: None,
            curr_sim_tick: 0,
            ui: GameUi::new(),
            viewport_size: vec2(0, 0),
            water: default(),
            console: Console::with_hotkey(K_CLI),
            effects: default(),
        }
    }

    /// 📺 Called whenever a new frame needs to be rendered.
    ///
    /// ⏱️ Advance the game state one frame:
    ///   * Apply given input events and new wall time `now_secs`.
    ///   * Advance the state one tick.
    ///   * Render state to `out` (scenegraph).
    pub fn tick_and_draw(&mut self, unix_micros: u64, events: impl Iterator<Item = InputEvent>, out: &mut Out) {
        self.now_micros = unix_micros;
        self.inputs.tick(&self.keymap, events);

        self.viewport_size = out.viewport_size; //👈
        out.camera_pos = self.camera_pos; //👈

        self.commands.extend(&mut self.inputs.drain_commands());

        self.update_fps(); // 👈 FPS is gamespeed independent
        self.exec_commands(); // 👈 exec commands even when paused (speed 0)

        if let Some(cmd) = self.console.tick(&self.inputs) {
            self.commands.push_back(cmd);
        }
        // 👇 📺 console overlays normal game. Disables game control when active.
        if !self.console.active {
            self.ui.update_and_draw(&mut self.inputs, out); // 👈 may consume inputs
            self.command_game();
        }

        if !self.paused {
            self.frame += 1;
            if self.frame % (16) == 0 {
                // 🪲 TODO: time major tick
                self.major_tick();
                self.water.major_tick(&self._tilemap); //👈 MAJOR
            } else {
                self.water.minor_tick(&self._tilemap); //👈 m i n o r
            }
        } else {
            // When paused: manually tick by pressing spacebar.
            // Handy for debugging.
            if self.inputs.just_pressed(K_SPACE) {
                self.frame = ((self.frame + 16) % 16); //🪲 bad pacing
                self.major_tick();
                self.water.major_tick(&self._tilemap);
                self.water.minor_tick(&self._tilemap);
            }
        }

        self.draw_world(out);
        self.console.draw(out);

        self.entities.gc();
    }

    pub(crate) fn major_tick(&mut self) {
        //self.tick_inspect(); //👈 must be first

        self.curr_sim_tick += 1;
        TICK_FOR_LOGGING.store(self.curr_sim_tick, std::sync::atomic::Ordering::Relaxed);
        self.tick_entities();
        self.tick_farmland();
        self.tick_renewables();
        self.update_text_overlay();

        self.sanity_check();
    }

    fn sanity_check(&mut self) {
        #[cfg(debug_assertions)]
        if let Err(e) = sanity_check(self) {
            log::error!("sanity check failed: {e}");
            self.last_sanity_error = Some(e.to_string());
            if self.debug.pause_on_sanity_failure {
                self.paused = true;
            }
        }
    }

    /// Update the text shown at the top of the screen:
    /// number of resources etc.
    fn update_text_overlay(&mut self) {
        self.header_text.clear();
        write!(&mut self.header_text, "{}", self.name).swallow_err();
        // print total resources
        let mut total_resources = [0u32; ResourceTyp::COUNT];
        for b in self.buildings() {
            for slot in b.inputs() {
                total_resources[slot.typ as usize] += slot.amount() as u32
            }
        }
        for res in ResourceTyp::all() {
            let count = total_resources[res as usize];
            write!(&mut self.header_text, " | {res:?}:{count}").swallow_err();
        }
    }

    fn tick_entities(&mut self) {
        self.entities.iter::<Pawn>().for_each(|p| p.tick(self));
        self.entities.iter::<Building>().for_each(|p| p.tick(self));
    }

    pub(crate) fn tick_renewables(&mut self) {
        let mut depleted = Vec::new();

        'outer: for (&tile, amount) in self.renewables.at_tile.0.borrow_mut().iter_mut() {
            // map tile determines which resource spawns here.
            let typ = match self.tile_at(tile) {
                Tile::GreyStone => ResourceTyp::Rock,
                bad => {
                    debug_assert!(false, "renewable on unknown tile: {bad} @ {tile}");
                    depleted.push(tile);
                    continue;
                }
            };
            // spawn at neighbors.
            for delta in [(-1, 0), (1, 0), (0, 1), (0, -1)] {
                let neigh = tile + vec2::from(delta);
                if self.tile_at(neigh).is_default_walkable() && self.resources.at(neigh).is_none() {
                    if *amount == 0 {
                        depleted.push(tile);
                        continue 'outer;
                    } else {
                        *amount -= 1;
                        self.resources.insert(neigh, typ);
                        continue;
                    }
                }
            }
        }
        // remove depleted resource tiles
        for tile in depleted {
            self.renewables.at_tile.0.borrow_mut().remove(&tile);
            self._tilemap.set(tile, Tile::Sand); // << TODO: base on underlying tile.
            self.effects.add_crater(self, tile);
        }
    }

    pub(crate) fn tick_farmland(&mut self) {
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
    pub(crate) fn is_walkable_by(&self, tile: vec2i16, pawn: &Pawn) -> bool {
        pawn.can_walk_on_tile(self.tile_at(tile))
        //if !tile.is_walkable(self.tile_at(tile)) {
        //    return false;
        //}
        // Cannot walk on buildings. 🪲 TODO: very inefficient
        //for b in self.buildings.iter() {
        //    if b.tile_bounds().contains(tile) && b.entrance() != tile {
        //        return false;
        //    }
        //}
        //true
    }

    // -------------------------------- Pawns

    // Add a pawn to the game and return it (now with `id` set).
    //pub fn spawn(&self, typ: PawnTyp, tile: vec2i16, team: Team) -> &Pawn {
    //    self.spawn_pawn(Pawn::new(typ, tile, team))
    //}

    //pub fn kill_pawn(&self, pawn: &Pawn) {
    //    trace!(pawn, "killed");
    //    self.entities.pawns.remove(pawn.id);
    //    //👇 keep worker list consistent
    //    pawn.home(self).map(|b| b.remove_dead_workers(self));
    //}

    //pub fn spawn(&self, pawn: Pawn) -> &Pawn {
    //    //log::trace!("spawn {:?} @ tile {}", pawn.typ, pawn.tile);
    //    //self.pawns.insert(pawn)
    //    self.entities.insert(pawn)
    //}

    /// TODO: simplify
    //    pub(crate) fn spawn_pawn_entity(&self, typ: PawnTyp, tile: Vector<i16, 2>, team: Team) -> Entity {
    //        //log::trace!("spawn {:?} @ tile {}", pawn.typ, pawn.tile);
    //        self.entities.insert(EntityStorage::pawn(typ, team, tile)).as_ref(self)
    //    }
    //    pub(crate) fn spawn_building_entity(&self, typ: BuildingTyp, tile: vec2i16, team: Team) -> Entity {
    //        //log::trace!("spawn {:?} @ tile {}", pawn.typ, pawn.tile);
    //        self.entities.insert(EntityStorage::building(typ, team, tile)).as_ref(self)
    //    }
    //
    // Pawn with given Id, if any.
    //pub fn pawn(&self, id: Id3) -> Option<&Pawn> {
    //    self.entities.pawns.get(id)
    //}

    /// Iterate over all Pawns.
    //pub fn pawns(&self) -> impl Iterator<Item = &Pawn> {
    //    self.entities.pawns.iter()
    //}

    /// Pawn at given position, if any.
    /// TODO: make faster via a hierarchy.
    pub fn pawn_at(&self, tile: vec2i16) -> Option<&Pawn> {
        self.pawns().find(|v| v.tile() == tile)
    }

    pub fn dyn_entities_at(&self, tile: vec2i16) -> impl Iterator<Item = Entity> {
        self.entities().filter(move |e| e.bounds().contains(tile))
    }

    pub fn entities_at<T: EntityT + HasTypeId>(&self, tile: vec2i16) -> impl Iterator<Item = &T> {
        self.entities.iter::<T>().filter(move |e| e.tile() == tile)
    }

    //pub(crate) fn building_entity_at(&self, tile: vec2i16) -> Option<BuildingRef> {
    //    // There can be multiple entities on one tile, but there should only be one building.
    //    self.entities_at(tile).filter_map(|e| e.downcast::<BuildingRef>()).next()
    //}

    /// Find nearest pawn inside given radius, where `f` is true.
    /// TODO: make faster via a hierarchy.
    pub fn find_entity(&self, around: vec2i16, radius: u16, f: impl Fn(Entity) -> bool) -> Option<Entity> {
        let radius = radius as i32;
        let radius2 = radius * radius;
        self //_
            .entities()
            .filter(|&p| p.tile().distance_squared(around) < radius2 && f(p))
            .min_by_key(|p| p.tile().distance_squared(around))
    }

    /// All currently selected Entity Ids.
    pub fn selected_entity_ids(&self) -> impl Iterator<Item = Id> {
        self.selected_entity_ids.iter()
    }

    // All currently selected Pawns.
    //pub fn selected_pawns(&self) -> impl Iterator<Item = &Pawn> {
    //    self.selected_pawn_ids.iter().filter_map(|id| self.pawn(id))
    //}

    /// All currently selected Entities.
    pub fn selected_entities(&self) -> impl Iterator<Item = Entity> {
        self.selected_entity_ids().filter_map(|id| self.entity(id))
    }

    /// All currently selected Entities of type `T`. E.g.
    ///     g.selected::<Pawn>()
    pub fn selected<T: EntityT + HasTypeId>(&self) -> impl Iterator<Item = &T> {
        self.selected_entity_ids().filter_map(|id| self.get::<T>(id))
    }

    // All currently selected Pawns.
    //pub(crate) fn selected_pawn_entities(&self) -> impl Iterator<Item = PawnRef> {
    //    self.selected_entities().filter_map(|e| e.downcast())
    //}

    /// Add Entity to selection.
    pub fn select_entity(&self, id: Id) {
        self.selected_entity_ids.insert(id);
    }

    // -------------------------------- Resources

    pub fn spawn_resource(&self, tile: impl Into<vec2i16>, resource: ResourceTyp) {
        self.resources.insert(tile.into(), resource);
    }

    // -------------------------------- Buildings

    /// Building with given Id, if any.
    pub fn building(&self, id: Id) -> Option<&Building> {
        self.entities.get::<Building>(id)
    }

    pub fn pawn(&self, id: Id) -> Option<&Pawn> {
        self.entities.get::<Pawn>(id)
    }

    /// Iterate over all Buildings.
    pub fn buildings(&self) -> impl Iterator<Item = &Building> {
        self.entities.iter::<Building>()
    }

    pub fn pawns(&self) -> impl Iterator<Item = &Pawn> {
        self.entities.iter::<Pawn>()
    }

    /// Building at given position, if any.
    /// TODO: make faster via a hierarchy.
    pub fn building_at(&self, tile: vec2i16) -> Option<&Building> {
        self.buildings().find(|v| v.bounds().contains_incl(tile))
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
        self.dt = (((self.now_micros - self.prev_frame_micros) as f64) / 1e6).clamp(0.001, 0.1); // clamp dt to 1-100ms to avoid craziness on clock suspend etc.
        self.prev_frame_micros = self.now_micros;
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
        if !self.tile_at(idx).is_default_walkable() {
            self.resources.remove(idx);
        }
    }

    pub(crate) fn deal_damage(&self, victim: Entity, amount: u8) {
        victim.get_health().saturating_sub(amount);
        if victim.health() == 0 {
            self.kill(victim);
        }
    }
}

impl GameCore for G {
    fn tick(&mut self, unix_micros: u64, events: impl Iterator<Item = InputEvent>, out: &mut Out) {
        self.tick_and_draw(unix_micros, events, out)
    }

    fn reset(&mut self) {
        *self = Self::test_world();
    }
}

impl Default for G {
    fn default() -> Self {
        Self::test_world()
    }
}
