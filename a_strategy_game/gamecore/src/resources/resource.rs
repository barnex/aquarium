use crate::prelude::*;

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash, TryFromPrimitive, Debug)]
#[repr(u8)]
pub enum ResourceTyp {
    Leaf = 1,
    Rock = 2,
    Dryweed = 3,
    Brick = 4,
    Ore = 5,
    // ⚠️👇 update `all()` below!
}

impl ResourceTyp {
    pub const MAX: Self = Self::Ore; // 👈⚠️ keep in sync! Use variant_count <https://github.com/rust-lang/rust/issues/73662> when stable
    pub const COUNT: usize = Self::MAX as usize + 1;

    pub fn all() -> impl Iterator<Item = Self> {
        let first = Self::Leaf;
        let last = Self::MAX;
        ((first as u8)..=(last as u8)).map(|i| Self::try_from_primitive(i).unwrap())
    }

    pub fn sprite(self) -> Sprite {
        use ResourceTyp::*;
        match self {
            Leaf => sprite!("leaf"),
            Rock => sprite!("rock"),
            Dryweed => sprite!("dryweed"),
            Brick => sprite!("brick"),
            Ore => sprite!("ore"),
        }
    }
}

impl Display for ResourceTyp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}
