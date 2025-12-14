use crate::prelude::*;

#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub struct Team(pub u8);

impl Team {
    pub const HUMAN1: Team = Team(1);
    pub const HUMAN2: Team = Team(2);
    pub const PESTS: Team = Team(100);

    /// Does this team attack `rhs`?
    pub(crate) fn is_hostile_to(self, rhs: Team) -> bool {
        self != rhs
    }
}

impl Display for Team {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Team").field(&self.0).finish()
    }
}
