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
        // 🥾
        if !self.is_at_destination() {
            self.walk_to_destination(g);
            return;
        }

        const NEAR_HOME: i64 = 4;

        // we are at some destination

        if let Some(home) = self.home(g) {
            // 🏭 home? deliver
            if self.tile == home.tile {
                self.deliver_cargo(home);
                self.go_to_near_resource(g);
            } else {
                // ✋☘️ at resource with hands free: take
                if self.cargo.is_none() {
                    // TODO: only suitable resources
                    self.cargo.set(g.resources.remove(self.tile.get()));
                }
                if self.cargo.is_some() {
                    self.go_home(g);
                } else {
                    self.go_to_near_resource(g).or_else(|| {
                        if home.tile.as_i32().distance_squared(self.tile().as_i32()) > NEAR_HOME*NEAR_HOME {
                            self.go_home(g);
                        };
                        OK
                    });
                }
            }
        }

        // 😴
        self.take_personal_space(g);
    }

    fn go_to_near_resource(&self, g: &G) -> Status {
        let new_dest = g.resources.iter().min_by_key(|(tile, res)| tile.distance_squared(self.tile.get())).map(|(tile, res)| tile)?;
        self.set_destination(g, new_dest);
        OK
    }

    fn go_home(&self, g: &G) -> Status {
        self.set_destination(g, g.building(self.home.get()?)?.entrance());
        OK
    }

    pub fn deliver_cargo(&self, home: &Building) -> Status {
        // 🪲 TODO: add to factory.
        log::error!("TODO: add to factory");
        self.cargo.take()?;
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
