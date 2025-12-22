use crate::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct Base {
    pub id: Id,
    pub tile: Cel<vec2i16>,
    pub health: Cel<u8>,
    pub team: Cel<Team>,
    pub sleep: Cel<u8>,
    pub traced: Cel<bool>,
}
impl Base {
    pub fn new(team: Team, tile: vec2i16) -> Self {
        Self {
            id: Id::INVALID,
            tile: tile.cel(),
            health: 1.cel(), // << TODO
            team: team.cel(),
            sleep: default(),
            traced: default(),
        }
    }

    pub(crate) fn set_id(&mut self, id: Id) {
        assert!(self.id == Id::INVALID);
        self.id = id;
    }
}

pub trait BaseT {
    fn g(&self) -> &G;
    fn base(&self) -> &Base;
    fn id(&self) -> Id {
        self.base().id
    }
    fn tile(&self) -> vec2i16 {
        self.base().tile.get()
    }
    fn set_tile(&self, tile: vec2i16) {
        // TODO: update spatial index
        self.base().tile.set(tile)
    }
    fn health(&self) -> u8 {
        self.base().health.get()
    }
    fn team(&self) -> Team {
        self.base().team.get()
    }
    fn traced(&self) -> &Cel<bool> {
        &self.base().traced
    }
    fn sleep(&self, ticks: u8) {
        trace!(self, "{ticks} ticks");
        self.base().sleep.set(ticks);
    }
    fn kill(&self) {
        trace!(self);
        self.g().entities.remove(self.id());
    }
}
