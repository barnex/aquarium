use crate::prelude::*;

/// Base functionality shared by all Entities.
#[derive(Serialize, Deserialize)]
pub struct Base {
    pub id: Id,
    pub tile: Cel<vec2i16>,
    pub sleep: Cel<u8>,
    pub team: Cel<Team>,
    pub health: Cel<u8>,
    pub traced: Cel<bool>,
}

impl Base {
    pub fn new(tile: vec2i16, team: Team, health: u8) -> Self {
        Self {
            id: Id::INVALID,
            tile: tile.cel(),
            sleep: 0.cel(),
            team: team.cel(),
            health: health.cel(),
            traced: false.cel(),
        }
    }

    #[inline]
    #[must_use = "tick must early return on true"]
    pub fn tick_sleep(&self) -> bool {
        if self.sleep.get() != 0 {
            self.sleep.saturating_sub(1);
            true
        } else {
            false
        }
    }
}

pub trait BaseT: Display {
    fn base(&self) -> &Base;

    #[inline]
    fn id(&self) -> Id {
        self.base().id
    }

    #[inline]
    fn tile(&self) -> vec2i16 {
        self.base().tile.get()
    }

    #[inline]
    fn get_tile(&self) -> &Cel<vec2i16> {
        &self.base().tile
    }

    #[inline]
    fn team(&self) -> Team {
        self.base().team.get()
    }

    #[inline]
    fn get_sleep(&self) -> &Cel<u8> {
        &self.base().sleep
    }

    #[inline]
    fn sleep(&self, ticks: u8) {
        trace!(self, "{ticks}");
        self.get_sleep().set(ticks);
    }

    #[inline]
    fn traced(&self) -> bool {
        self.base().traced.get()
    }

    #[inline]
    fn get_traced(&self) -> &Cel<bool> {
        &self.base().traced
    }

    #[inline]
    fn health(&self) -> u8 {
        self.base().health.get()
    }

    #[inline]
    fn get_health(&self) -> &Cel<u8> {
        &self.base().health
    }
}

//fn set_id3(&mut self, id: Id) {
//    self.id = id
//}
//fn id(&self) -> Id {
//    self.id
//}
