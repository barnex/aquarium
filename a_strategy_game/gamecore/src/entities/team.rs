use crate::prelude::*;

#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub struct Team(u8);

impl Team {
    pub const HUMAN1: Team = Team(1);
    pub const HUMAN2: Team = Team(2);
    pub const PESTS: Team = Team(100);
}
