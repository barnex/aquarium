use crate::prelude::*;

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash, TryFromPrimitive, Debug)]
#[repr(u8)]
pub enum ResourceTyp {
    Leaf = 1,
    Rock = 2,
    // ⚠️👇 update `all()` below!
}

impl ResourceTyp {
    pub fn all() -> impl Iterator<Item = Self> {
        let first = Self::Leaf;
        let last = Self::Rock; // 👈⚠️ keep in sync!
        ((first as u8)..=(last as u8)).map(|i| Self::try_from_primitive(i).unwrap())
    }

    pub fn sprite(self) -> Sprite {
        use ResourceTyp::*;
        match self {
            Leaf => sprite!("leaf"),
            Rock => sprite!("rock"),
        }
    }
}
