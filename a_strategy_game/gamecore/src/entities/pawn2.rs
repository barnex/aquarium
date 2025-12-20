use crate::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct Pawn2Ext {
    // move
    pub route: Route,

    // work
    pub home: Cel<Option<Id>>,
    pub cargo: Cel<Option<ResourceTyp>>,

    // attack
    pub target: Cel<Option<Id>>,
    pub rot: Cel<f32>,
}

pub struct Pawn2<'g> {
    pub base: &'g Base,
    pub ext: &'g Pawn2Ext,
}

impl<'g> Pawn2<'g> {
    pub fn tick(&self) {
        println!("hello from soldier @ {}", self.tile())
    }
}

impl<'g> BaseT for Pawn2<'g> {
    fn base(&self) -> &Base {
        &self.base
    }
}

impl<'g> Deref for Pawn2<'g> {
    type Target = Pawn2Ext;

    fn deref(&self) -> &Self::Target {
        &self.ext
    }
}

impl Into<Ext> for Pawn2Ext {
    fn into(self) -> Ext {
        Ext::Pawn(self)
    }
}
