use crate::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct BuildingExt {
    pub workers: CSet<Id>,
    pub _downstream: CSet<Id>,
    pub _upstream: CSet<Id>,
    pub resources: [Cel<u16>; MAX_RES_SLOTS],
}

pub struct BuildingRef<'g> {
    pub g: &'g G,
    pub base: &'g Base,
    pub ext: &'g BuildingExt,
}

impl<'g> BuildingRef<'g> {
    pub fn tick(&self) {
        println!("hello from building @ {}", self.tile())
    }

    pub fn draw(&self, out: &mut Out) {
        let sprite = sprite!("TODO");
        out.draw_sprite_rot(L_SPRITES, sprite, self.tile().pos(), /*rot=*/ 0.0);
    }
}

impl<'g> BaseT for BuildingRef<'g> {
    fn base(&self) -> &Base {
        &self.base
    }
}

impl<'g> Deref for BuildingRef<'g> {
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
