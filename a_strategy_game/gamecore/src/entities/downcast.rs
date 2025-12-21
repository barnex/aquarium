use crate::prelude::*;

impl<'g> Entity<'g> {
    //pub fn downcast_building(&self) -> Option<BuildingRef<'g>> {
    //    BuildingRef::downcast(self)
    //}

    pub fn downcast<T: Downcast<'g>>(&self) -> Option<T> {
        T::downcast(&self)
    }
}

pub trait Downcast<'g>: Sized {
    fn downcast(supr: &Entity<'g>) -> Option<Self>;
}

impl<'g> Downcast<'g> for BuildingRef<'g> {
    fn downcast(supr: &Entity<'g>) -> Option<Self> {
        match supr.ext {
            Ext::Building(ext) => Some(BuildingRef { g: supr.g, base: supr.base, ext }),
            _ => None,
        }
    }
}

impl<'g> Downcast<'g> for PawnRef<'g> {
    fn downcast(supr: &Entity<'g>) -> Option<Self> {
        match supr.ext {
            Ext::Pawn(ext) => Some(PawnRef { g: supr.g, base: supr.base, ext }),
            _ => None,
        }
    }
}
