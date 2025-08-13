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
    Pollen = 2,
    Cat = 3,
    Crablet = 4,
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
            PawnTyp::Pollen => sprite!("pollen"),
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

        // we are at some destination

        if let Some(home) = self.home(g) {
            // 🏭 home? deliver
            if self.tile == home.tile {
                self.deliver_cargo(home);

                // find new target
                if let Some(new_dest) = g.resources.iter().min_by_key(|(tile, res)| tile.distance_squared(self.tile.get())).map(|(tile, res)| tile) {
                    self.set_destination(g, new_dest);
                }
            } else {
                // ✋☘️ at resource with hands free: take
                if self.cargo.is_none() {
                    // TODO: only suitable resources
                    self.cargo.set(g.resources.remove(self.tile.get()));
                }
                if self.cargo.is_none() {
                    // wtf, someone stole it.
                    // find nearby or go home?
                }
                self.set_destination(g, home.tile);
            }
        }

        // 😴
        self.take_personal_space(g);
    }

    pub fn deliver_cargo(&self, home: &Building) {
        // 🪲 TODO: add to factory.
        log::error!("TODO: add to factory");
        self.cargo.take();
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
        //TODO: self.route.clear();
    }

    fn start_route_to(&self, g: &G, dest: vec2i16) {
        let max_dist = 100;
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
