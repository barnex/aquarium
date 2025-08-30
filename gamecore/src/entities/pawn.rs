use crate::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Pawn {
    pub id: Id,
    pub typ: PawnTyp,
    pub tile: Cel<vec2i16>,
    pub route: Route,
    pub home: Cel<Option<Id>>,
    pub cargo: Cel<Option<ResourceTyp>>,
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
        log::trace!("🦀 at destination");

        // 🏭 for worker pawns
        if let Some(home) = self.home(g) {
            if self.cargo.is_some() {
                self.tick_with_cargo(g, home);
            } else {
                self.tick_without_cargo(g, home);
            }
        }

        // const NEAR_HOME: i64 = 4;

        // 😴
        self.take_personal_space(g);
    }

    /// 📦 Carrying something
    fn tick_with_cargo(&self, g: &G, home: &Building) {
        // 🏭 We are home: try to drop off

        log::trace!("🦀 have cargo");

        if let Some(building) = g.building_at(self.tile()) {
            log::trace!("🦀 at building: try deliver");

            self.deliver_cargo(building);
            // 📦 drop-off failed because destination is full:
            // deliver downstream
            if let Some(res) = self.cargo.get() {
                log::trace!("🦀 deliver failed, look for downstream");
                if let Some(downstream) = home //_
                    .downstream_buildings(g)
                    .filter(|b| b.has_resource_slot(res))
                    // TODO: nearest, should have actual free slot
                    // TODO: chain home
                    .next()
                {
                    log::trace!("🦀 moving to downstream {:?}@{}", downstream.typ, downstream.id);
                    self.set_destination(g, downstream.tile);
                }
            } else {
                // successful home delivery :)
                log::trace!("🦀 successful home delivery, thinking");

                if home.is_full() {
                    log::trace!("🦀 home is full");
                    for downstream in home
                        .downstream_buildings(g) //_
                        .sorted_by_key(|d| d.tile.distance_squared(home.tile))
                    {
                        for (res, slot, _) in home.resource_slots(){
                            if downstream.can_accept_resource(res){
                                self.cargo.set(home.take_resource(res));
                                self.set_destination(g, downstream.entrance());
                            }
                        }
                    }
                }
            }
        } else {
            log::trace!("🦀 not at building: going home");
            self.go_home(g);
        }
    }

    /// ✋ Hands free
    fn tick_without_cargo(&self, g: &G, home: &Building) {
        log::trace!("🦀 no cargo");
        const NEAR_HOME: i64 = 4;

        // Try picking up resource
        if let Some(res) = g.resources.at(self.tile()) {
            log::trace!("🦀 standing on {res:?}");
            if home.has_resource_slot(res) {
                return self.cargo.set(g.resources.remove(self.tile.get()));
            } else {
                // ...
            }
        } else {
            log::trace!("🦀 no resource here");

            if self.go_to_near_resource(g).is_some() {
                return;
            };

            if home.tile.as_i32().distance_squared(self.tile().as_i32()) > NEAR_HOME * NEAR_HOME {
                self.go_home(g);
            }
        }
    }

    fn go_to_near_resource(&self, g: &G) -> Status {
        log::trace!("🦀 go to near resource?");
        let home = self.home(g)?;
        let new_dest = g.resources.iter().filter(|(_, res)| home.can_accept_resource(*res)).min_by_key(|(tile, _)| tile.distance_squared(self.tile.get())).map(|(tile, _)| tile)?;
        self.set_destination(g, new_dest);
        OK
    }

    fn go_home(&self, g: &G) -> Status {
        log::trace!("🦀 going home");
        self.set_destination(g, g.building(self.home.get()?)?.entrance());
        OK
    }

    pub fn deliver_cargo(&self, home: &Building) -> Status {
        // 🪲 TODO: add to factory.
        if let Some(resource) = self.cargo.take() {
            match home.add_resource(resource) {
                OK => (),
                FAIL => self.cargo.set(Some(resource)), // TODO: go sleep a bit or so
            }
        }
        OK
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

    pub fn set_destination(&self, g: &G, dest: vec2i16) {
        if !self.can_move() {
            return;
        }
        self.start_route_to(g, dest);
    }

    fn start_route_to(&self, g: &G, dest: vec2i16) {
        let max_dist = 42;
        let distance_map = DistanceMap::new(dest, max_dist, |p| g.is_walkable(p));
        if let Some(path) = distance_map.path_to_center(self.tile.get()) {
            self.route.set(path);
        }
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
