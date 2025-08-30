use crate::*;

// TODO: Nonzero
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Id {
    pub(super) index: u32,
    pub(super) generation: u32,
}

impl Id {
    pub fn is_valid(&self) -> bool {
        *self != Self::default()
    }
}

impl Debug for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Self as Display>::fmt(self, f)
    }
}

impl Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:04x}.{:04x}", self.index, self.generation)
    }
}

impl Default for Id {
    fn default() -> Self {
        Self { index: 0, generation: 0 }
    }
}
