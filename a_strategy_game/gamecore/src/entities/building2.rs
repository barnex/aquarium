use crate::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct BuildingExt {
    pub workers: CSet<Id>,
    pub _downstream: CSet<Id>,
    pub _upstream: CSet<Id>,
    pub resources: [Cel<u16>; MAX_RES_SLOTS],
}

pub struct Building2<'g> {
    pub base: &'g Base,
    pub ext: &'g BuildingExt,
}

impl<'g> Building2<'g> {
    pub fn tick(&self) {
        println!("hello from building @ {}", self.tile())
    }
}

impl<'g> BaseT for Building2<'g> {
    fn base(&self) -> &Base {
        &self.base
    }
}

impl<'g> Deref for Building2<'g> {
    type Target = BuildingExt;

    fn deref(&self) -> &Self::Target {
        &self.ext
    }
}

impl Into<Ext> for BuildingExt {
    fn into(self) -> Ext {
        Ext::Building(self)
    }
}
