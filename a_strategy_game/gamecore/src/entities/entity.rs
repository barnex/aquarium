use crate::prelude::*;

pub trait EntityT: Any + BaseT + Debug + 'static + HasId3 {
    fn draw(&self, g: &G, out: &mut Out);
    fn can_move(&self) -> bool;
    fn size(&self) -> vec2u8 {
        //👇 Default size
        vec2(1, 1)
    }
    fn bounds(&self) -> Bounds2Di16 {
        Bounds2D::with_size(self.tile(), self.size().map(|v| v as i16))
    }
    // Center pixel in world coordinates
    fn center(&self) -> vec2i {
        self.bounds().map(|tile| tile.pos()).center()
    }
}

#[derive(Clone, Copy)]
/// Logically a `&'g dyn EntityT`, but with details hidden.
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
