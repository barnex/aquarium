use crate::prelude::*;

pub const MAX_RES_SLOTS: usize = 4;

#[derive(Serialize, Deserialize, Debug)]
pub struct Building {
    pub id: Id,
    pub typ: BuildingTyp,
    pub tile: vec2i16,
    pub team: Team,

    pub workers: CSet<Id>,
    pub _downstream: CSet<Id>,
    pub _upstream: CSet<Id>,
    resources: [Cel<u16>; MAX_RES_SLOTS],
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash, TryFromPrimitive, Debug)]
#[repr(u8)]
pub enum BuildingTyp {
    HQ = 1,
    Farm = 2,
    Quarry = 3,
    StarNest = 4,
    // 👆 ⚠️ keep in sync!
}

impl BuildingTyp {
    pub fn all() -> impl Iterator<Item = Self> {
        let first = Self::HQ;
        let last = Self::StarNest; // 👈⚠️ keep in sync! Use variant_count <https://github.com/rust-lang/rust/issues/73662> when stable.
        ((first as u8)..=(last as u8)).map(|i| Self::try_from_primitive(i).unwrap())
    }

    pub fn sprite(&self) -> Sprite {
        use BuildingTyp::*;
        match self {
            HQ => sprite!("hq"),
            Farm => sprite!("shell_big"),
            Quarry => sprite!("quarry"),
            StarNest => sprite!("starnest"),
        }
    }

    /// Index in Building.resrouces and max capacity.
    /// 0 unused :(
    fn _resource_metadata(self) -> [Option<(usize, u16)>; ResourceTyp::COUNT] {
        match self {
            BuildingTyp::HQ => [None, Some((0, 100)), Some((1, 100))],
            BuildingTyp::Farm => [None, Some((0, 20)), None],
            BuildingTyp::Quarry => [None, None, Some((0, 30))],
            BuildingTyp::StarNest => [None, Some((0, 100)), None],
        }
    }

    pub fn default_workers(self) -> (PawnTyp, usize) {
        match self {
            BuildingTyp::HQ => (PawnTyp::Cat, 2),
            BuildingTyp::Farm => (PawnTyp::Cat, 1),
            BuildingTyp::Quarry => (PawnTyp::Cat, 1),
            BuildingTyp::StarNest => (PawnTyp::Starfish, 10),
        }
    }

    pub(crate) fn can_build_on(self, tile: Tile) -> bool {
        use BuildingTyp::*;
        use Tile::*;
        match (self, tile) {
            (StarNest, Water) => true,
            (_, tile) => tile.is_default_walkable(),
        }
    }
}

impl Building {
    //-------------------------------------------------------------------------------- spawn/init
    pub fn new(typ: BuildingTyp, tile: impl Into<Vector<i16, 2>>, team: Team) -> Self {
        Self {
            id: default(),
            tile: tile.into(),
            team,
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
        log::trace!("spawn default workers for {self}");
        let (pawntyp, num) = self.typ.default_workers();

        for _ in 0..num {
            let pawn = g.spawn(pawntyp, self.tile, self.team);
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

    /// Building size in tiles. E.g. 3x3.
    pub fn size(&self) -> vec2u8 {
        use BuildingTyp::*;
        match self.typ {
            HQ => (3, 3),
            Farm => (2, 2),
            Quarry => (2, 2),
            StarNest => (3, 3),
        }
        .into()
    }

    pub fn tile_bounds(&self) -> Bounds2Di16 {
        Bounds2D::with_size(self.tile, self.size().as_i16())
    }

    pub fn entrance(&self) -> vec2i16 {
        self.tile // TODO
    }

    pub fn id(&self) -> Id {
        self.id
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
    for building in g.buildings().filter(|b| b.id != hq.id) {
        let my_resources = building.iter_resources().map(|(r, _)| r).collect::<HashSet<_>>();
        let neighbors = g
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

/// For Memkeep::insert().
impl SetId for Building {
    fn set_id(&mut self, id: Id) {
        self.id = id;
    }
}

impl Display for Building {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}{}", self.typ, self.id)
    }
}
