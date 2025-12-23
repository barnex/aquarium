use crate::prelude::*;

pub trait EntityT: Any + HasId3 + Debug + 'static {
    fn draw(&self, g: &G, out: &mut Out);
    fn tile(&self) -> vec2i16;
    fn team(&self) -> Team;
    fn can_move(&self) -> bool;
    fn bounds(&self) -> Bounds2Di16 {
        //👇 Default size
        Bounds2D::with_size(self.tile(), vec2(1, 1))
    }
}

#[derive(Clone, Copy)]
pub struct Entity<'g>(&'g dyn EntityT);

impl<'g> Entity<'g> {
    pub fn downcast<T: Any>(self) -> Option<&'g T> {
        (self.0 as &dyn Any).downcast_ref()
    }
}

impl<'g> Deref for Entity<'g> {
    type Target = dyn EntityT;

    fn deref(&self) -> &'g Self::Target {
        self.0
    }
}

impl<'g> From<&'g Pawn> for Entity<'g> {
    fn from(v: &'g Pawn) -> Self {
        Self(v)
    }
}
impl<'g> From<&'g Building> for Entity<'g> {
    fn from(v: &'g Building) -> Self {
        Self(v)
    }
}

impl<'g> Debug for Entity<'g> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
