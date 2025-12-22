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

pub struct PawnRef<'g> {
    pub g: &'g G,
    pub base: &'g Base,
    pub ext: &'g Pawn2Ext,
}

impl Pawn2Ext {
    pub(crate) fn new(typ: PawnTyp) -> Self {
        Self {
            typ,
            route: default(),
            home: default(),
            cargo: default(),
            target: default(),
            rot: default(),
        }
    }
}

impl<'g> EntityT for PawnRef<'g> {
    fn tick(&self) {
        if !self.route.is_finished() {
            self.walk_to_destination();
            return;
        }
    }

    fn draw(&self, out: &mut Out) {
        let sprite = self.typ.sprite(self.team());
        out.draw_sprite_rot(L_SPRITES, sprite, self.tile().pos(), self.rot.get());
        if let Some(res) = self.cargo.get() {
            out.draw_sprite(L_SPRITES + 1, res.sprite(), self.tile().pos() + vec2(0, 8));
        }
    }

    fn size(&self) -> vec2u8 {
        vec2(1, 1)
    }

    fn can_move(&self) -> bool {
        self.typ.can_move()
    }
}

impl<'g> PawnRef<'g> {
    /// Find path to `dest` and start moving.
    pub(crate) fn set_destination(&self, dest: vec2i16) {
        let max_dist = 42;
        log::warn!("todo: is_walkable");
        //let distance_map = DistanceMap::new(dest, max_dist, |p| self.g().is_walkable_by(p, self));
        let distance_map = DistanceMap::new(dest, max_dist, |p| true); // <<<<<<< TODO
        if let Some(path) = distance_map.path_to_center(self.tile()) {
            trace!(self, "dest={dest} path len={}", path.len());
            self.route.set(path);
        } else {
            trace!(self, "no path")
        }
    }

    /// Take one step towards destination, if any.
    fn walk_to_destination(&self) {
        if let Some(next_tile) = self.route.next() {
            log::warn!("todo: is_walkable");
            //if g.is_walkable_by(next_tile, self) {
            self.set_tile(next_tile);
            //} else {
            //    // TODO: handle destination unreachable
            //    self.route.clear(); // ☹️
            //}
        }
    }

    pub fn can_assign_to(&self, building: &BuildingRef) -> bool {
        if !self.typ.is_worker() {
            //trace!(self, "assign {self} to {building}: is not a worker");
            return false;
        }
        if self.team() != building.team() {
            //trace!(self, "assign {self} to {building}: wrong team: {} != {}", self.team, building.team);
            return false;
        }
        true
    }
}

impl<'g> BaseT for PawnRef<'g> {
    fn base(&self) -> &Base {
        &self.base
    }
    fn g(&self) -> &G {
        &self.g
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
