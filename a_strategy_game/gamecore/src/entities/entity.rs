use crate::prelude::*;

pub trait Entity: Debug + GetId3 + 'static {
    fn draw(&self, g: &G, out: &mut Out);
    fn tile(&self) -> vec2i16;
    fn team(&self) -> Team;
    fn can_move(&self) -> bool;
    fn bounds(&self) -> Bounds2Di16 {
        //👇 Default size
        Bounds2D::with_size(self.tile(), vec2(1, 1))
    }
}
