use crate::prelude::*;

//struct Hex {
//    soldiers: Vec<(Base, SoldierExt)>,
//    buidlings: Vec<(Base, BuildingExt)>,
//}

// Like `Box<dyn EntityT>`.
#[derive(Serialize, Deserialize)]
pub struct Entity {
    pub base: Base,
    pub ext: Ext,
}

// Like `&'g dyn EntityT`
pub struct EntityRef<'g> {
    pub g: &'g G,
    pub base: &'g Base,
    pub ext: &'g Ext,
}

pub trait EntityT: BaseT {
    fn tick(&self);
    fn draw(&self, out: &mut Out);
}

impl<'g> BaseT for EntityRef<'g> {
    fn base(&self) -> &Base {
        self.base
    }
}

impl<'g> EntityT for EntityRef<'g> {
    fn draw(&self, out: &mut Out) {
        match &self.ext {
            Ext::Pawn(ext) => PawnRef { g: self.g, base: &self.base, ext }.draw(out),
            Ext::Building(ext) => BuildingRef { g: self.g, base: &self.base, ext }.draw(out),
        }
    }
    fn tick(&self) {
        match &self.ext {
            Ext::Pawn(ext) => PawnRef { g: self.g, base: &self.base, ext }.tick(),
            Ext::Building(ext) => BuildingRef { g: self.g, base: &self.base, ext }.tick(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Base {
    id: Id,
    tile: Cel<vec2i16>,
    health: Cel<u8>,
    team: Cel<Team>,
    sleep: Cel<u8>,
    traced: Cel<bool>,
}

pub trait BaseT {
    fn base(&self) -> &Base;
    fn id(&self) -> Id {
        self.base().id
    }
    fn tile(&self) -> vec2i16 {
        self.base().tile.get()
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
}

impl BaseT for Entity {
    fn base(&self) -> &Base {
        &self.base
    }
}

#[derive(Serialize, Deserialize)]
pub enum Ext {
    Pawn(Pawn2Ext),        // pawn2.rs
    Building(BuildingExt), // building2.rs
}

impl Entity {
    pub fn new(tile: vec2i16, team: Team, ext: impl Into<Ext>) -> Self {
        Self {
            base: Base {
                id: Id::default(),
                tile: tile.cel(),
                health: 100.cel(),
                team: team.cel(),
                sleep: default(),
                traced: default(),
            },
            ext: ext.into(),
        }
    }
}

impl SetId for Entity {
    fn set_id(&mut self, id: Id) {
        self.base.id = id
    }
}

impl Display for Entity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "E{}", self.id())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_it() {
        let tile = vec2(1, 2);
        let soldier = Entity::new(
            tile,
            Team::Red,
            Pawn2Ext {
                typ: PawnTyp::Crab,
                route: default(),
                home: default(),
                cargo: default(),
                target: default(),
                rot: default(),
            },
        );
        let building = Entity::new(
            tile + 1,
            Team::Red,
            BuildingExt {
                workers: default(),
                _downstream: default(),
                _upstream: default(),
                resources: default(),
            },
        );
        //soldier.tick();
        //building.tick();
    }
}
