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
    IronMine = 7,
    CoalMine = 8,
    // 👆 ⚠️ keep in sync!
}

impl BuildingTyp {
    pub fn all() -> impl Iterator<Item = Self> {
        let first = Self::HQ;
        let last = Self::CoalMine; // 👈⚠️ keep in sync! Use variant_count <https://github.com/rust-lang/rust/issues/73662> when stable.
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
            IronMine => sprite!("quarry"), // TODO
            CoalMine => sprite!("quarry"), // TODO
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
            IronMine => (2, 2),
            CoalMine => (2, 2),
        }
        .into()
    }

    pub fn input_resources(self) -> &'static [(ResourceTyp, u16)] {
        use ResourceTyp::*;
        match self {
            BuildingTyp::HQ => &[(Dryweed, 1000), (Brick, 1000), (Iron, 1000)],
            BuildingTyp::Farm => &[(Leaf, 10)],
            BuildingTyp::Quarry => &[(Rock, 10)],
            BuildingTyp::StarNest => &[(Leaf, 100)],
            BuildingTyp::FoodPacker => &[(Leaf, 10)],
            BuildingTyp::RockPacker => &[(Rock, 10)],
            BuildingTyp::IronMine => &[(Ore, 10), (Coal, 10)],
            BuildingTyp::CoalMine => &[(Coal, 10)],
        }
    }

    pub fn output_resources(self) -> &'static [(ResourceTyp, u16)] {
        use ResourceTyp::*;
        match self {
            BuildingTyp::HQ => &[],
            BuildingTyp::Farm => &[(Dryweed, 10)],
            BuildingTyp::Quarry => &[(Brick, 10)],
            BuildingTyp::StarNest => &[],
            BuildingTyp::FoodPacker => &[],
            BuildingTyp::RockPacker => &[],
            BuildingTyp::IronMine => &[(Iron, 10)],
            BuildingTyp::CoalMine => &[],
        }
    }

    pub fn default_workers(self) -> (PawnTyp, usize) {
        use PawnTyp::*;
        match self {
            BuildingTyp::HQ => (Cat, 2),
            BuildingTyp::Farm => (Cat, 1),
            BuildingTyp::Quarry => (Cat, 1),
            BuildingTyp::StarNest => (Starfish, 10),
            BuildingTyp::FoodPacker => (Cat, 1),
            BuildingTyp::RockPacker => (Cat, 1),
            BuildingTyp::IronMine => (Cat, 2),
            BuildingTyp::CoalMine => (Cat, 2),
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
