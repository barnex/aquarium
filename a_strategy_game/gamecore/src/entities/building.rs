use crate::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct Building {
    base: Base,

    typ: BuildingTyp,
    workers: CSet<Id>,
    inputs: SmallVec<ResourceSlot, 3>,
    outputs: SmallVec<ResourceSlot, 2>,
}

/// An factory input or output slot. Can hold up to some maximum amount of resources.
/// E.g. `Leaf: 7 out of 100`
#[derive(Serialize, Deserialize)]
pub struct ResourceSlot {
    /// Type of Resource stored. E.g. Leaf, Rock.
    pub typ: ResourceTyp,
    /// Current amount stored. Always <= max.
    pub amount: Cel<u16>,
    /// Maximum amount stored.
    pub max: u16,
}

impl ResourceSlot {
    fn new(typ: ResourceTyp, max: u16) -> Self {
        debug_assert!(max > 0);
        Self { typ, max, amount: 0.cel() }
    }

    pub fn is_full(&self) -> bool {
        self.amount() >= self.max
    }

    pub fn fullness_pct(&self) -> u32 {
        (self.amount() as u32 * 100) / (self.max as u32)
    }

    pub fn is_empty(&self) -> bool {
        self.amount() == 0
    }

    /// Try to take one resource, return it if successful or None otherwise (slot was empty).
    pub fn try_take_one(&self) -> Option<ResourceTyp> {
        if self.amount() > 0 {
            self.amount.sub(1);
            Some(self.typ)
        } else {
            None
        }
    }

    /// Slot has at least `n` items. So we can successfully `take(n)`.
    pub fn has_at_least(&self, n: u16) -> bool {
        self.amount() >= n
    }

    pub fn take(&self, n: u16) -> Option<()> {
        debug_assert!(self.has_at_least(n));
        if self.has_at_least(n) {
            self.amount.sub(n);
            OK
        } else {
            FAIL
        }
    }

    pub fn can_accept(&self, n: u16) -> bool {
        self.amount() + n <= self.max
    }

    pub fn add_one(&self) -> Option<()> {
        if self.amount() < self.max {
            self.amount.inc(1);
            OK
        } else {
            FAIL
        }
    }

    #[inline]
    pub fn amount(&self) -> u16 {
        self.amount.get()
    }

    /// For internal use/debug only.
    pub(crate) fn get_amount(&self) -> &Cel<u16> {
        &self.amount
    }
}

impl BaseT for Building {
    fn base(&self) -> &Base {
        &self.base
    }
}

impl HasId3 for Building {
    fn set_id3(&mut self, id: Id) {
        self.base.id = id
    }
}

impl EntityT for Building {
    fn on_spawned(&self, g: &G) {
        trace!(self);
        self.spawn_default_workers(g);
        //update_downstream_buildings(g); // << inefficient
    }

    fn on_killed(&self, g: &G) {
        trace!(self);

        //🏠 unlink workers from home
        self.workers().iter().filter_map(|id| g.pawn(id)).for_each(|p| p.home.set(None));

        //💥 draw crater effect
        let bounds = self.bounds();
        let footprint = cross(bounds.x_range(), bounds.y_range());
        for tile in footprint {
            g.effects.add_crater(g, tile.into());
        }
    }

    fn draw(&self, _: &G, out: &mut Out) {
        let building = self;
        // 🏭 Building sprite
        out.draw_sprite(L_SPRITES, building.typ.sprite(), building.tile().pos());

        // ☘️ Resource amounts
        let vstride = 18; // some fiddly offsets to make it look better
        let mut cursor = building.tile().pos() - vec2(4, 4);
        for slot in building.inputs().chain(building.outputs()) {
            out.draw_sprite(L_SPRITES + 1, slot.typ.sprite(), cursor - vec2(0, 4));
            out.draw_text(L_SPRITES + 1, &format!("{}/{}", slot.amount, slot.max), cursor + vec2::EX * TILE_ISIZE);
            cursor[1] += vstride;
        }
    }

    fn can_move(&self) -> bool {
        false
    }

    /// Building size in tiles. E.g. 3x3.
    fn size(&self) -> vec2u8 {
        self.typ.size()
    }
}

impl Building {
    pub const MAX_RES_SLOTS: usize = 4;

    //-------------------------------------------------------------------------------- spawn/init
    pub fn new(typ: BuildingTyp, tile: impl Into<vec2i16>, team: Team) -> Self {
        let tile = tile.into();
        Self {
            base: Base::new(tile, team, typ.default_health()),
            typ,
            workers: default(),
            inputs: typ.input_resources().iter().map(|&(typ, max)| ResourceSlot::new(typ, max)).collect(),
            outputs: typ.output_resources().iter().map(|&(typ, max)| ResourceSlot::new(typ, max)).collect(),
        }
    }

    //-------------------------------------------------------------------------------- building function

    pub fn is_full(&self) -> bool {
        self.inputs.iter().all(|slot| slot.is_full())
    }

    /// Depots accept resources transferred from other buildings.
    /// E.g. a Farm is *not* a depot as it only harvests fresh food,
    /// whereas headquarters is a huge depot that receives only from others.
    pub fn is_depot(&self) -> bool {
        match self.typ {
            BuildingTyp::HQ => true,
            BuildingTyp::Farm => false,
            BuildingTyp::Quarry => false,
            BuildingTyp::StarNest => false,
            BuildingTyp::FoodPacker => true,
            BuildingTyp::RockPacker => true,
        }
    }

    /// Is building interested in this resource (pending capacity)?
    pub fn has_input(&self, res: ResourceTyp) -> bool {
        self.input(res).is_some()
    }

    pub fn has_output(&self, res: ResourceTyp) -> bool {
        self.output(res).is_some()
    }

    /// Is there capacity to receive one resource of given type (e.g. one rock)?
    pub fn can_accept(&self, res: ResourceTyp) -> bool {
        match self.input(res) {
            None => false,
            Some(slot) => !slot.is_full(),
        }
    }

    pub fn can_provide(&self, res: ResourceTyp) -> bool {
        match self.output(res) {
            None => false,
            Some(slot) => !slot.is_empty(),
        }
    }

    pub fn add_resource(&self, res: ResourceTyp) -> Status {
        trace!(self, "{res}");
        self.input(res)?.add_one()
    }

    pub fn take_resource(&self, res: ResourceTyp) -> Option<ResourceTyp> {
        trace!(self, "{res}");
        self.input(res)?.try_take_one()
    }

    /// Current number and maximum capacity for given resource.
    pub fn input(&self, typ: ResourceTyp) -> Option<&ResourceSlot> {
        self.inputs().find(|slot| slot.typ == typ)
    }

    pub fn output(&self, typ: ResourceTyp) -> Option<&ResourceSlot> {
        self.outputs().find(|slot| slot.typ == typ)
    }

    ///
    /// Current number and maximum capacity for given resource.
    pub fn inputs(&self) -> impl Iterator<Item = &ResourceSlot> {
        self.inputs.iter()
    }

    pub fn outputs(&self) -> impl Iterator<Item = &ResourceSlot> {
        self.outputs.iter()
    }

    pub fn can_spawn(&self, g: &G) -> bool {
        debug_assert!(self.id() == Id::INVALID, "can_spawn check is only useful pre-insert");

        //❓check if building fits here
        let bounds = self.bounds();
        let mut footprint = cross(bounds.x_range(), bounds.y_range());

        let is_buildable = |tile: vec2i16, typ: BuildingTyp| {
            if !typ.can_build_on(g.tile_at(tile)) {
                return false;
            }
            for b in g.buildings() {
                if b.bounds().contains(tile) {
                    return false;
                }
            }
            true
        };

        footprint.all(|(x, y)| is_buildable(vec2(x, y), self.typ))
    }

    pub fn tick(&self, g: &G) {
        if self.base.tick_sleep() {
            return;
        }

        use ResourceTyp::*;
        match self.typ {
            BuildingTyp::HQ => (),
            BuildingTyp::Farm => self.tick_factory(3, Leaf, Dryweed, 50),
            BuildingTyp::Quarry => self.tick_factory(3, Rock, Brick, 50),
            BuildingTyp::StarNest => self.tick_star_nest(g),
            BuildingTyp::FoodPacker => self.tick_factory(3, Leaf, Dryweed, 50),
            BuildingTyp::RockPacker => self.tick_factory(3, Rock, Brick, 50),
        }
    }

    fn tick_factory(&self, from_n: u16, from: ResourceTyp, to: ResourceTyp, ticks: u8) {
        let from_slot = self.input(from).unwrap();
        let to_slot = self.output(to).unwrap();
        if (from_slot.amount() >= from_n) && !to_slot.is_full() {
            trace!(self, "produce :)");
            from_slot.take(from_n);
            to_slot.add_one();
            self.sleep(ticks);
        }
    }

    pub fn workers(&self) -> &CSet<Id> {
        &self.workers
    }

    /// Building::init -> spawn the workers for this building.
    fn spawn_default_workers(&self, g: &G) {
        let (pawntyp, num) = self.typ.default_workers();
        trace!(self, "{num} x {pawntyp:?}");

        for _ in 0..num {
            let pawn = g.spawn(Pawn::new(pawntyp, self.tile(), self.team()));
            pawn.assign_to(g, self);
        }
    }

    fn tick_star_nest(&self, g: &G) {
        // TODO: delay
        if g.tick % 128 == 0 {
            if self.workers.is_empty() {
                self.spawn_default_workers(g);
            }
        }
        // slowly drain resources
        if g.tick % 16 == 0 {
            if self.inputs[0].has_at_least(1) {
                self.inputs[0].take(1);
                self.get_health().clamped_add(self.typ.default_health(), 1);
            }
        }
    }

    //pub fn tile_bounds(&self) -> Bounds2Di16 {
    //    Bounds2D::with_size(self.tile(), self.size().as_i16())
    //}

    pub fn entrance(&self) -> vec2i16 {
        self.tile() // TODO
    }
}

//🪲 Remove for now. Not correctly sync. Just calculate as needed.
//fn update_downstream_buildings(g: &G) {
//    log::trace!("update downstream buildings");
//    let Some(hq) = g.buildings().find(|b| b.typ == BuildingTyp::HQ) else { return log::error!("No HQ") };
//
//    // 🪲 TODO: quadratic in #buildings. Use spatial queries instead.
//    const MAX_DIST2: i32 = 30 * 30; // TODO
//    for building in g.buildings().filter(|b| b.id() != hq.id()) {
//        let my_resources = building.outputs().map(|slot| slot.typ).collect::<HashSet<_>>();
//        let neighbors = g
//            .buildings() //_
//            .filter(|b| b.id() != building.id())
//            .filter(|b| b.is_depot())
//            .filter(|b| b.tile().distance_squared(building.tile()) < MAX_DIST2)
//            .filter(|b| b.tile().distance_squared(hq.tile()) < building.tile().distance_squared(hq.tile()))
//            .filter(|b| b.inputs.iter().map(|slot| slot.typ).any(|r| my_resources.contains(&r)))
//            .map(|b| b.id());
//        building._downstream.clear();
//        building._downstream.extend(neighbors);
//    }
//
//    // upstream
//    // for building in self.buildings() {
//    //     let my_resources = building.iter_resources().map(|(r, _)| r).collect::<HashSet<_>>();
//    //     let neighbors = self
//    //         .buildings() //_
//    //         .filter(|b| b.id != building.id)
//    //         .filter(|b| b.tile.distance_squared(building.tile) < MAX_DIST2)
//    //         .filter(|b| b.tile.distance_squared(hq.tile) >= building.tile.distance_squared(hq.tile))
//    //         .filter(|b| b.iter_resources().map(|(r, _)| r).any(|r| my_resources.contains(&r)))
//    //         .map(|b| b.id);
//    //     building._downstream.clear();
//    //     building._downstream.extend(neighbors);
//    // }
//}

impl Display for Building {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}{}", self.typ, self.id())
    }
}

impl Debug for Building {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        pretty_print(f, self)
    }
}
