use crate::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Pawn {
    pub id: Id,
    pub typ: PawnTyp,
    pub tile: Cel<vec2i16>,
    pub route: Route,
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash, TryFromPrimitive, Debug)]
#[repr(u8)]
pub enum PawnTyp {
    Leaf = 1,
    Pollen = 2,
    Cat = 3,
    Crablet = 4,
    // ⚠️👇 update `all()` below!
}
impl PawnTyp {
    pub fn all() -> impl Iterator<Item = Self> {
        let first = Self::Leaf;
        let last = Self::Crablet; // 👈⚠️ keep in sync!
        ((first as u8)..=(last as u8)).map(|i| Self::try_from_primitive(i).unwrap())
    }
}

impl PawnTyp {
    pub fn sprite(&self) -> Sprite {
        match self {
            PawnTyp::Leaf => sprite!("leaf"),
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
        }
    }

    pub(crate) fn tick(&self, g: &G) {
        if !self.is_at_destination() {
            self.walk_to_destination(g);
            return;
        }

        self.take_personal_space(g);
    }

    /// If standing on another pawn, move aside randomly.
    fn take_personal_space(&self, g: &G) {
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
        use PawnTyp::*;
        match self.typ {
            Leaf => false,
            _ => true,
        }
    }

    fn can_move(&self) -> bool {
        use PawnTyp::*;
        match self.typ {
            Leaf => false,
            _ => true,
        }
    }

    fn walk_to_destination(&self, g: &G) {
        if let Some(next_tile) = self.route.next() {
            if g.is_walkable(next_tile) {
                self.tile.set(next_tile);
            } else {
                // TODO: handle destination unreachable
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
        let distance_map = DistanceMap::new(dest, 254, |p| g.is_walkable(p));
        if let Some(path) = distance_map.path_to_center(self.tile.get()) {
            self.route.set(path);
        }
    }

    pub fn bounds(&self) -> Bounds2Di {
        Bounds2D::with_size(self.tile.pos(), vec2::splat(TILE_ISIZE))
    }

    pub fn center(&self) -> vec2i {
        self.bounds().center()
    }


    pub fn is_at_destination(&self) -> bool {
        self.route.is_finished()
    }
}

// For MemKeep::insert.
impl SetId for Pawn {
    fn set_id(&mut self, id: Id) {
        self.id = id;
    }
}
