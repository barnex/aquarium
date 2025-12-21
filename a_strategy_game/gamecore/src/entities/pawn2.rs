use crate::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct Pawn2Ext {
    pub typ: PawnTyp,

    // move
    pub route: Route,

    // work
    pub home: Cel<Option<Id>>,
    pub cargo: Cel<Option<ResourceTyp>>,

    // attack
    pub target: Cel<Option<Id>>,
    pub rot: Cel<f32>,
}

impl<'g> EntityT for PawnRef<'g> {
    fn tick(&self) {
        println!("hello from soldier @ {}", self.tile())
    }

    fn draw(&self, out: &mut Out) {
        let sprite = self.typ.sprite(self.team());
        out.draw_sprite_rot(L_SPRITES, sprite, self.tile().pos(), self.rot.get());
        if let Some(res) = self.cargo.get() {
            out.draw_sprite(L_SPRITES + 1, res.sprite(), self.tile().pos() + vec2(0, 8));
        }
    }
}

pub struct PawnRef<'g> {
    pub g: &'g G,
    pub base: &'g Base,
    pub ext: &'g Pawn2Ext,
}

impl<'g> BaseT for PawnRef<'g> {
    fn base(&self) -> &Base {
        &self.base
    }
}

impl<'g> Deref for PawnRef<'g> {
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
