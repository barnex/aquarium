use crate::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct Building {
    base: Base,

    pub typ: BuildingTyp,
    pub workers: CSet<Id>,
    pub _downstream: CSet<Id>,
    pub _upstream: CSet<Id>,
    resources: [Cel<u16>; Self::MAX_RES_SLOTS],
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
    }

    fn draw(&self, g: &G, out: &mut Out) {
        let building = self;
        // 🏭 Building sprite
        out.draw_sprite(L_SPRITES, building.typ.sprite(), building.tile().pos());

        // ☘️ Resource amounts
        let vstride = 18; // some fiddly offsets to make it look better
        let mut cursor = building.tile().pos() - vec2(4, 4);
        for (typ, count) in building.iter_resources() {
            out.draw_sprite(L_SPRITES + 1, typ.sprite(), cursor - vec2(0, 4));
            out.draw_text(L_SPRITES + 1, &format!("{count}"), cursor + vec2::EX * TILE_ISIZE);
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
    pub fn new(typ: BuildingTyp, tile: impl Into<Vector<i16, 2>>, team: Team) -> Self {
        let tile = tile.into();
        Self {
            base: Base::new(tile, team, typ.default_health()),
            typ,
            workers: default(),
            resources: default(),
            _downstream: default(),
            _upstream: default(),
        }
    }

    /// Called post-insert to initialized whatever is needed after building.
    pub fn init(&self, g: &G) {
        log::trace!("init {self}");
        update_downstream_buildings(g);
        self.spawn_default_workers(g);
    }

    /// Building::init -> spawn the workers for this building.
    fn spawn_default_workers(&self, g: &G) {
        let (pawntyp, num) = self.typ.default_workers();
        trace!(self, "{num} x {pawntyp:?}");

        for _ in 0..num {
            let pawn = g.spawn(Pawn::new(pawntyp, self.tile(), self.team()));
            g.assign_to(pawn, self);
        }
    }

    pub fn tick(&self, g: &G) {
        match self.typ {
            BuildingTyp::HQ => (),
            BuildingTyp::Farm => (),
            BuildingTyp::Quarry => (),
            BuildingTyp::StarNest => self.tick_star_nest(g),
        }
    }

    fn tick_star_nest(&self, g: &G) {
        // TODO: delay
        if g.tick % 128 == 0 {
            self.remove_dead_workers(g);
            if self.workers.is_empty() {
                self.spawn_default_workers(g);
            }
        }
        // slowly drain resources
        if g.tick % 16 == 0 {
            self.resources[0].saturating_sub(1);
        }
    }

    //-------------------------------------------------------------------------------- building function
    pub fn is_full(&self) -> bool {
        self.resource_slots().all(|(_, slot, cap)| slot.get() >= cap)
    }

    pub fn downstream_buildings<'g>(&self, g: &'g G) -> impl Iterator<Item = &'g Building> {
        self._downstream.iter().filter_map(|id| g.building(id))
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
        }
    }

    /// Is building interested in this resource (pending capacity)?
    pub fn has_resource_slot(&self, res: ResourceTyp) -> bool {
        self.resource_slot(res).is_some()
    }

    /// Is there capacity to receive one resource of given type (e.g. one rock)?
    pub fn can_accept_resource(&self, res: ResourceTyp) -> bool {
        match self.resource_slot(res) {
            None => false,
            Some((count, cap)) => count.get() < cap,
        }
    }

    pub fn add_resource(&self, res: ResourceTyp) -> Status {
        let (slot, cap) = self.resource_slot(res)?;
        if slot.get() < cap {
            debug_assert!(self.can_accept_resource(res));
            slot.inc(1);
            OK
        } else {
            FAIL
        }
    }

    pub fn take_resource(&self, res: ResourceTyp) -> Option<ResourceTyp> {
        let (slot, _) = self.resource_slot(res)?;
        if slot.get() > 0 {
            slot.sub(1);
            Some(res)
        } else {
            None
        }
    }

    /// Current number and maximum capacity for given resource.
    pub fn resource_slot(&self, res: ResourceTyp) -> Option<(&Cel<u16>, u16)> {
        let (i, cap) = self.typ._resource_metadata()[res as usize]?;
        Some((&self.resources[i], cap))
    }

    /// Iterate Resource type, current amount, and maximum capacity.
    pub fn resource_slots(&self) -> impl Iterator<Item = (ResourceTyp, &Cel<u16>, u16)> {
        self.typ
            ._resource_metadata()
            .into_iter()
            .enumerate()
            .filter_map(|(r, v)| v.map(|(i, cap)| (r, i, cap)))
            .map(|(r, i, cap)| (ResourceTyp::try_from_primitive(r as u8).unwrap(), &self.resources[i], cap))
    }

    pub fn iter_resources(&self) -> impl Iterator<Item = (ResourceTyp, u16)> {
        // TODO: reverse index instead of linear search
        self.typ
            ._resource_metadata()
            .into_iter()
            .enumerate()
            .filter_map(|(r, v)| v.map(|(i, _cap)| (r, i)))
            .map(|(r, i)| (ResourceTyp::try_from_primitive(r as u8).unwrap(), i))
            .map(|(r, i)| (r, self.resources[i as usize].get()))
    }

    //pub fn tile_bounds(&self) -> Bounds2Di16 {
    //    Bounds2D::with_size(self.tile(), self.size().as_i16())
    //}

    pub fn entrance(&self) -> vec2i16 {
        self.tile() // TODO
    }

    pub(crate) fn remove_dead_workers(&self, g: &G) {
        self.workers.retain(|&id| g.pawn(id).is_some());
    }
}

fn update_downstream_buildings(g: &G) {
    log::trace!("update downstream buildings");
    let Some(hq) = g.buildings().find(|b| b.typ == BuildingTyp::HQ) else { return log::error!("No HQ") };

    // 🪲 TODO: quadratic in #buildings. Use spatial queries instead.
    const MAX_DIST2: i32 = 30 * 30; // TODO
    for building in g.buildings().filter(|b| b.id() != hq.id()) {
        let my_resources = building.iter_resources().map(|(r, _)| r).collect::<HashSet<_>>();
        let neighbors = g
            .buildings() //_
            .filter(|b| b.id() != building.id())
            .filter(|b| b.is_depot())
            .filter(|b| b.tile().distance_squared(building.tile()) < MAX_DIST2)
            .filter(|b| b.tile().distance_squared(hq.tile()) < building.tile().distance_squared(hq.tile()))
            .filter(|b| b.iter_resources().map(|(r, _)| r).any(|r| my_resources.contains(&r)))
            .map(|b| b.id());
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
