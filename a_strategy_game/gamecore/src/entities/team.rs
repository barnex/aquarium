use crate::prelude::*;

#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub enum Team {
    Pests = 0,
    Red = 1,
    Blue = 2,
}

impl Team {
    /// Does this team attack `rhs`?
    pub(crate) fn is_hostile_to(self, rhs: Team) -> bool {
        self != rhs
    }
}

impl Display for Team {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self, f)
    }
}
