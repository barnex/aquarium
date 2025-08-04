use crate::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Pawn {
    pub id: Id,
    pub typ: PawnTyp,
    pub tile: Cel<vec2i16>,
    pub dest: Cel<vec2i16>,
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
            dest: tile.cel(),
            route: default(),
        }
    }

    pub(crate) fn tick(&self, g: &State) {
        if !self.is_at_destination(){
            self.walk_to_destination(g);
            return;
        }
    }

    fn teleport_to(&self, g: &State, dst: vec2i16) {
        self.tile.set(dst);
    }
    
    fn walk_to_destination(&self, g: &State){
        if let Some(next_tile) = self.route.next(){
            self.tile.set(next_tile);
        }else{
            if ! self.is_at_destination(){
                self.compute_route(g);
            }
        }
    }
    
    fn compute_route(&self, g: &State){
        let distance_map = DistanceMap::new(self.dest.get(), 254, |p| g.is_walkable(p));
        if let Some(path) = distance_map.path_to_center(self.tile.get()){
            self.route.set(path);
        }
    }
    
    pub fn bounds(&self) -> Bounds2Di {
        Bounds2D::with_size(self.tile.pos(), vec2::splat(TILE_ISIZE))
    }

    pub fn center(&self) -> vec2i {
        self.bounds().center()
    }

    pub fn set_destination(&self, dest: vec2i16) {
        self.dest.set(dest);
    }

    pub fn is_at_destination(&self) -> bool {
        self.tile == self.dest
    }
}

// For MemKeep::insert.
impl SetId for Pawn {
    fn set_id(&mut self, id: Id) {
        self.id = id;
    }
}
