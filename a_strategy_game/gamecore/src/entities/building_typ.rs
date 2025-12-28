use crate::prelude::*;

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash, TryFromPrimitive, Debug)]
#[repr(u8)]
pub enum BuildingTyp {
    HQ = 1,
    Farm = 2,
    Quarry = 3,
    StarNest = 4,
    FoodPacker = 5,
    RockPacker = 6,
    // 👆 ⚠️ keep in sync!
}

impl BuildingTyp {
    pub fn all() -> impl Iterator<Item = Self> {
        let first = Self::HQ;
        let last = Self::RockPacker; // 👈⚠️ keep in sync! Use variant_count <https://github.com/rust-lang/rust/issues/73662> when stable.
        ((first as u8)..=(last as u8)).map(|i| Self::try_from_primitive(i).unwrap())
    }

    pub fn sprite(&self) -> Sprite {
        use BuildingTyp::*;
        match self {
            HQ => sprite!("hq"),
            Farm => sprite!("shell_big"),
            Quarry => sprite!("quarry"),
            StarNest => sprite!("starnest"),
            FoodPacker => sprite!("factory"),
            RockPacker => sprite!("factory"),
        }
    }

    /// Footprint size in tiles.
    pub fn size(&self) -> vec2u8 {
        use BuildingTyp::*;
        match self {
            HQ => (3, 3),
            Farm => (2, 2),
            Quarry => (2, 2),
            StarNest => (3, 3),
            FoodPacker => (3, 3),
            RockPacker => (3, 3),
        }
        .into()
    }

    /// Index in Building.resrouces and max capacity.
    /// 0 unused :(
    pub fn _resource_metadata(self) -> [Option<(usize, u16)>; ResourceTyp::COUNT] {
        match self {
            BuildingTyp::HQ => [None, None, None, Some((0, 1000)), Some((1, 1000))],
            BuildingTyp::Farm => [None, Some((0, 20)), None, None, None],
            BuildingTyp::Quarry => [None, None, Some((0, 30)), None, None],
            BuildingTyp::StarNest => [None, Some((0, 100)), None, None, None],
            BuildingTyp::FoodPacker => [None, Some((0, 10)), None, Some((2, 10)), None],
            BuildingTyp::RockPacker => [None, None, Some((1, 10)), None, Some((3, 10))],
        }
    }

    pub fn default_workers(self) -> (PawnTyp, usize) {
        match self {
            BuildingTyp::HQ => (PawnTyp::Cat, 2),
            BuildingTyp::Farm => (PawnTyp::Cat, 1),
            BuildingTyp::Quarry => (PawnTyp::Cat, 1),
            BuildingTyp::StarNest => (PawnTyp::Starfish, 10),
            BuildingTyp::FoodPacker => (PawnTyp::Cat, 1),
            BuildingTyp::RockPacker => (PawnTyp::Cat, 1),
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

    pub fn default_health(&self) -> u8 {
        100
    }
}
