use crate::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct BuildingExt {
    pub typ: BuildingTyp,
    pub workers: CSet<Id>,
    pub _downstream: CSet<Id>,
    pub _upstream: CSet<Id>,
    pub resources: [Cel<u16>; MAX_RES_SLOTS],
}

impl BuildingExt {
    pub(crate) fn new(typ: BuildingTyp) -> Self {
        Self {
            typ,
            workers: default(),
            _downstream: default(),
            _upstream: default(),
            resources: default(),
        }
    }
}

pub struct BuildingRef<'g> {
    pub g: &'g G,
    pub base: &'g Base,
    pub ext: &'g BuildingExt,
}

impl<'g> EntityT for BuildingRef<'g> {
    fn tick(&self) {
        println!("hello from building @ {}", self.tile())
    }

    fn draw(&self, out: &mut Out) {
        let sprite = self.typ.sprite();
        out.draw_sprite_rot(L_SPRITES, sprite, self.tile().pos(), /*rot=*/ 0.0);
    }

    fn size(&self) -> vec2u8 {
        self.typ.size()
    }

    fn g(&self) -> &G {
        &self.g
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
