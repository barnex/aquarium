use crate::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Pawn {
    pub id: Id,
    pub typ: PawnTyp,
    pub tile: Cel<vec2i16>,
    pub route: Route,
    pub home: Cel<Option<Id>>,
    pub cargo: Cel<Option<ResourceTyp>>,
    pub traced: Cel<bool>,
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash, TryFromPrimitive, Debug)]
#[repr(u8)]
pub enum PawnTyp {
    Kitten = 1,
    Cat = 2,
    Crablet = 3,
    // ⚠️👇 update `all()` below!
}
impl PawnTyp {
    pub fn all() -> impl Iterator<Item = Self> {
        let first = Self::Kitten;
        let last = Self::Crablet; // 👈⚠️ keep in sync!
        ((first as u8)..=(last as u8)).map(|i| Self::try_from_primitive(i).unwrap())
    }
}

impl PawnTyp {
    pub fn sprite(&self) -> Sprite {
        match self {
            PawnTyp::Kitten => sprite!("kit7"),
            PawnTyp::Cat => sprite!("kit4"),
            PawnTyp::Crablet => sprite!("ferris"),
        }
    }
}

impl Pawn {
    pub fn new(typ: PawnTyp, tile: vec2i16) -> Self {
        Self {
            id: Id::default(),
            typ,
            tile: tile.cel(),
            route: default(),
            home: None.cel(),
            cargo: None.cel(),
            traced: false.cel(),
        }
    }

    // ⏱️
    pub(crate) fn tick(&self, g: &G) {
        // 🥾 always first go where you were going
        if !self.is_at_destination() {
            self.walk_to_destination(g);
            return;
        }

        // 📍 now you are at destination

        // 🏭 for worker pawns
        if let Some(home) = self.home(g) {
            self.tick_delivery_work(g, home)
        }

        // 😴
        self.take_personal_space(g);
    }

    fn tick_delivery_work(&self, g: &G, home: &Building) {
        let on_building = g.building_at(self.tile());
        let on_home = on_building.map(Building::id) == Some(home.id);

        match on_building {
            _ if on_home => self.tick_on_home(g, home),
            Some(building) => self.tick_on_other_building(g, home, building),
            None => self.tick_away_from_building(g, home),
        }
    }

    ///   +home+
    ///   | 🦀 |    ☘️️️ ☘️️
    ///   +----+
    fn tick_on_home(&self, g: &G, home: &Building) {
        trace!(self, "on home");
        self.try_deliver_cargo(home);
        match self.cargo() {
            None => self.go_to_near_resource(g, home).or_else(|| self.move_resource_downstream(g, home)),
            Some(_) => self.go_downstream(g, home),
        };
    }

    /// +-HQ-+   +home+
    /// | 🦀 |   |    |    ☘️️️ ☘️️
    /// +----+   +----+
    fn tick_on_other_building(&self, g: &G, home: &Building, building: &Building) {
        trace!(self, "on other building: {:?}", building.typ);
        self.try_deliver_cargo(building);
        match self.cargo() {
            None => self.go_to_near_resource(g, home).or_else(|| self.go_home(g)),
            Some(_) => self.go_home(g),
        };
    }

    ///   +home+
    ///   |    |    🦀 ☘️️
    ///   +----+
    fn tick_away_from_building(&self, g: &G, home: &Building) {
        trace!(self, "away from any building");
        self.try_pick_up_cargo(g, home);
        match self.cargo() {
            Some(_) => self.go_home(g),
            None => self.go_to_near_resource(g, home).or_else(|| self.go_home(g)),
        };
    }

    fn go_to_near_resource(&self, g: &G, home: &Building) -> Status {
        trace!(self, "go to near resource?");
        let new_dest = g.resources.iter().filter(|(_, res)| home.can_accept_resource(*res)).min_by_key(|(tile, _)| tile.distance_squared(self.tile.get())).map(|(tile, _)| tile)?;
        self.set_destination(g, new_dest);
        OK
    }

    fn go_home(&self, g: &G) -> Status {
        trace!(self, "going home");
        self.set_destination(g, g.building(self.home.get()?)?.entrance());
        OK
    }

    fn go_downstream(&self, g: &G, home: &Building) -> Status {
        trace!(self, "going downstream");
        debug_assert!(self.cargo.is_some());

        let cargo = self.cargo()?;

        for downstream in home
            .downstream_buildings(g) //_
            .sorted_by_key(|d| d.tile.distance_squared(self.tile()))
        {
            if downstream.can_accept_resource(cargo) {
                self.set_destination(g, downstream.entrance());
                return OK;
            }
        }
        FAIL
    }

    fn move_resource_downstream(&self, g: &G, home: &Building) -> Status {
        trace!(self, "take any resource downstream");
        debug_assert!(self.cargo.is_none());

        if self.cargo.is_some() {
            return FAIL;
        }

        for downstream in home
            .downstream_buildings(g) //_
            .sorted_by_key(|d| d.tile.distance_squared(home.tile))
        {
            for (res, slot, _) in home.resource_slots() {
                if slot.get() > 0 && downstream.can_accept_resource(res) {
                    self.cargo.set(home.take_resource(res));
                    if self.set_destination(g, downstream.entrance()).is_some() {
                        trace!(self, "take {:?} downstream to {:?}", self.cargo(), downstream.typ);
                        return OK;
                    }
                }
            }
        }
        FAIL
    }

    pub fn try_deliver_cargo(&self, building: &Building) -> Status {
        trace!(self, "try deliver cargo {:?} to {:?}", self.cargo, building.typ);

        let resource = self.cargo.take()?;
        match building.add_resource(resource) {
            OK => OK,
            FAIL => {
                // TODO: go sleep a bit or so
                self.cargo.set(Some(resource));
                FAIL
            }
        }
    }

    pub fn try_pick_up_cargo(&self, g: &G, home: &Building) -> Status {
        let res = g.resources.at(self.tile())?;
        //trace!(self, "try_pick_up_cargo", self.cargo, building.typ);
        if home.can_accept_resource(res) {
            self.cargo.set(g.resources.remove(self.tile()));
            OK
        } else {
            FAIL
        }
    }

    fn steal_any_resource(&self, g: &G, home: &Building, building: &Building) -> Status {
        debug_assert!(self.home.get() == Some(home.id));

        for (res, slot, _) in building.resource_slots() {
            if slot.get() > 0 && home.can_accept_resource(res) {
                self.cargo.set(building.take_resource(res));
                if self.set_destination(g, home.entrance()).is_some() {
                    trace!(self, "take {:?} home to {:?}", self.cargo(), home.typ);
                    return OK;
                }
            }
        }
        FAIL
    }

    pub fn home<'a>(&self, g: &'a G) -> Option<&'a Building> {
        g.buildings.get_maybe(self.home.get())
    }

    /// If standing on another pawn, move aside randomly.
    fn take_personal_space(&self, g: &G) {
        if !self.can_move() {
            return;
        }
        let standing_on_other = g.pawns().filter(|p| p.id != self.id).find(|p| p.tile == self.tile).is_some();
        if standing_on_other {
            self.teleport_to(g, self.tile.get() + g.random_vec_incl::<i16>(-1..=1));
        }
    }

    fn teleport_to(&self, g: &G, dst: vec2i16) {
        if g.is_walkable(dst) {
            self.tile.set(dst);
            self.route.clear();
        }
    }

    fn is_commandable(&self) -> bool {
        match self.typ {
            _ => true,
        }
    }

    fn can_move(&self) -> bool {
        match self.typ {
            _ => true,
        }
    }

    fn walk_to_destination(&self, g: &G) {
        if let Some(next_tile) = self.route.next() {
            if g.is_walkable(next_tile) {
                self.tile.set(next_tile);
            } else {
                // TODO: handle destination unreachable
                self.route.clear(); // ☹️
            }
        }
    }

    pub fn set_destination(&self, g: &G, dest: vec2i16) -> Status {
        if !self.can_move() {
            return FAIL;
        }
        self.start_route_to(g, dest)
    }

    fn start_route_to(&self, g: &G, dest: vec2i16) -> Status {
        let max_dist = 42;
        let distance_map = DistanceMap::new(dest, max_dist, |p| g.is_walkable(p));
        let path = distance_map.path_to_center(self.tile())?;
        self.route.set(path);
        OK
    }

    pub fn destination(&self) -> Option<vec2i16> {
        self.route.destination()
    }

    pub fn bounds(&self) -> Bounds2Di {
        Bounds2D::with_size(self.tile.pos(), vec2::splat(TILE_ISIZE))
    }

    pub fn tile(&self) -> vec2i16 {
        self.tile.get()
    }

    pub fn cargo(&self) -> Option<ResourceTyp> {
        self.cargo.get()
    }

    pub fn center(&self) -> vec2i {
        self.bounds().center()
    }

    pub fn is_at_destination(&self) -> bool {
        self.route.is_finished()
    }

    pub fn crab(tile: impl Into<vec2i16>) -> Self {
        Self::new(PawnTyp::Crablet, tile.into())
    }
}

impl Display for Pawn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}{}@{}", self.typ, self.id, self.tile())
    }
}

// For MemKeep::insert.
impl SetId for Pawn {
    fn set_id(&mut self, id: Id) {
        self.id = id;
    }
}
