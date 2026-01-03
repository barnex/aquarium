use crate::prelude::*;

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash, TryFromPrimitive, Debug)]
#[repr(u8)]
pub enum PawnTyp {
    Kitten = 1,
    Cat = 2,
    Crab = 3,
    Turret = 4,
    Starfish = 5,
    // ⚠️👇 update `all()` below!
}
impl PawnTyp {
    pub fn all() -> impl Iterator<Item = Self> {
        let first = Self::Kitten;
        let last = Self::Starfish; // 👈⚠️ keep in sync!
        ((first as u8)..=(last as u8)).map(|i| Self::try_from_primitive(i).unwrap())
    }

    pub fn can_walk_on(&self, tile: Tile) -> bool {
        use PawnTyp::*;
        use Tile::*;
        match (self, tile) {
            (Starfish, Water | Canal) => true,
            (Turret, Block) => true,
            (_, tile) => tile.is_default_walkable(),
        }
    }

    pub fn can_move(self) -> bool {
        match self {
            PawnTyp::Kitten => true,
            PawnTyp::Cat => true,
            PawnTyp::Crab => true,
            PawnTyp::Turret => false,
            PawnTyp::Starfish => true,
        }
    }

    /// Worker Pawns can be assigned to work at a factory,
    /// and transport resources.
    pub fn is_worker(self) -> bool {
        let is_worker = match self {
            PawnTyp::Kitten => true,
            PawnTyp::Cat => true,
            PawnTyp::Crab => false,
            PawnTyp::Turret => false,
            PawnTyp::Starfish => true,
        };
        debug_assert!(if is_worker { self.can_move() } else { true }, "worker must be able to move");
        is_worker
    }

    pub fn default_health(self) -> u8 {
        match self {
            PawnTyp::Kitten => 3,
            PawnTyp::Cat => 5,
            PawnTyp::Crab => 5,
            PawnTyp::Turret => 20,
            PawnTyp::Starfish => 4,
        }
    }
}

impl PawnTyp {
    pub fn sprite(&self, team: Team) -> Sprite {
        use PawnTyp::*;
        use Team::*;
        match (self, team) {
            (Kitten, _) => sprite!("kit7"),
            (Cat, Red) => sprite!("Manneke_06"),
            (Cat, _) => sprite!("kit3"),
            (Crab, Blue) => sprite!("Manneke_40"),
            (Crab, _) => sprite!("Manneke_42"),
            (Turret, Red) => sprite!("turret"),
            (Turret, Blue) => sprite!("turretblue"),
            (Turret, _) => sprite!("turret"),
            (Starfish, _) => sprite!("starfish"),
        }
    }
}
