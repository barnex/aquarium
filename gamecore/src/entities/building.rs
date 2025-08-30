use crate::prelude::*;

const MAX_RES_SLOTS: usize = 4;

#[derive(Serialize, Deserialize, Debug)]
pub struct Building {
    pub id: Id,
    pub typ: BuildingTyp,
    pub tile: vec2i16,
    pub workers: CSet<Id>,
    pub _downstream: CSet<Id>,
    resources: [Cel<u16>; MAX_RES_SLOTS],
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash, TryFromPrimitive, Debug)]
#[repr(u8)]
pub enum BuildingTyp {
    HQ = 1,
    Farm = 2,
    Quarry = 3,
    // 👆 ⚠️ keep in sync!
}

impl BuildingTyp {
    pub fn all() -> impl Iterator<Item = Self> {
        let first = Self::HQ;
        let last = Self::Quarry; // 👈⚠️ keep in sync! Use variant_count <https://github.com/rust-lang/rust/issues/73662> when stable.
        ((first as u8)..=(last as u8)).map(|i| Self::try_from_primitive(i).unwrap())
    }

    pub fn sprite(&self) -> Sprite {
        use BuildingTyp::*;
        match self {
            HQ => sprite!("hq"),
            Farm => sprite!("shell_big"),
            Quarry => sprite!("quarry"),
        }
    }
}

impl Building {
    pub fn new(typ: BuildingTyp, tile: impl Into<Vector<i16, 2>>) -> Self {
        Self {
            id: default(),
            typ,
            tile: tile.into(),
            workers: default(),
            resources: default(),
            _downstream: default(),
        }
    }

    pub fn is_full(&self) -> bool {
        self.resource_slots().all(|(_, slot, cap)| slot.get() >= cap)
    }

    pub fn downstream_buildings<'g>(&self, g: &'g G) -> impl Iterator<Item = &'g Building> {
        self._downstream.iter().filter_map(|id| g.building(id))
    }

    /// Depots accept resources transferred from other buildings.
    /// E.g. a Farm is *not* a depot as it only harvests fresh food,
    /// whereas headquarters is a huge depot that receives only from others.
    pub fn is_depot(&self) -> bool {
        match self.typ {
            BuildingTyp::HQ => true,
            BuildingTyp::Farm => false,
            BuildingTyp::Quarry => false,
        }
    }

    /// Is building interested in this resource (pending capacity)?
    pub fn has_resource_slot(&self, res: ResourceTyp) -> bool {
        self.resource_slot(res).is_some()
    }

    /// Is there capacity to receive one resource of given type (e.g. one rock)?
    pub fn can_accept_resource(&self, res: ResourceTyp) -> bool {
        match self.resource_slot(res) {
            None => false,
            Some((count, cap)) => count.get() < cap,
        }
    }

    pub fn add_resource(&self, res: ResourceTyp) -> Status {
        let (slot, cap) = self.resource_slot(res)?;
        if slot.get() < cap {
            debug_assert!(self.can_accept_resource(res));
            slot.inc(1);
            OK
        } else {
            FAIL
        }
    }

    pub fn take_resource(&self, res: ResourceTyp) -> Option<ResourceTyp> {
        let (slot, _) = self.resource_slot(res)?;
        if slot.get() > 0 {
            slot.sub(1);
            Some(res)
        } else {
            None
        }
    }

    /// Current number and maximum capacity for given resource.
    pub fn resource_slot(&self, res: ResourceTyp) -> Option<(&Cel<u16>, u16)> {
        let (i, cap) = Self::_resource_metadata(self.typ)[res as usize]?;
        Some((&self.resources[i], cap))
    }

    /// Iterate Resource type, current amount, and maximum capacity.
    pub fn resource_slots(&self) -> impl Iterator<Item = (ResourceTyp, &Cel<u16>, u16)> {
        Self::_resource_metadata(self.typ)
            .into_iter()
            .enumerate()
            .filter_map(|(r, v)| v.map(|(i, cap)| (r, i, cap)))
            .map(|(r, i, cap)| (ResourceTyp::try_from_primitive(r as u8).unwrap(), &self.resources[i], cap))
    }

    pub fn iter_resources(&self) -> impl Iterator<Item = (ResourceTyp, u16)> {
        // TODO: reverse index instead of linear search
        Self::_resource_metadata(self.typ)
            .into_iter()
            .enumerate()
            .filter_map(|(r, v)| v.map(|(i, _cap)| (r, i)))
            .map(|(r, i)| (ResourceTyp::try_from_primitive(r as u8).unwrap(), i))
            .map(|(r, i)| (r, self.resources[i as usize].get()))
    }

    /// Index in Building.resrouces and max capacity.
    /// 0 unused :(
    fn _resource_metadata(typ: BuildingTyp) -> [Option<(usize, u16)>; ResourceTyp::COUNT] {
        match typ {
            BuildingTyp::HQ => [None, Some((0, 100)), Some((1, 100))],
            BuildingTyp::Farm => [None, Some((0, 20)), None],
            BuildingTyp::Quarry => [None, None, Some((0, 30))],
        }
    }

    /// Building size in tiles. E.g. 3x3.
    pub fn size(&self) -> vec2u8 {
        use BuildingTyp::*;
        match self.typ {
            HQ => (3, 3),
            Farm => (2, 2),
            Quarry => (2, 2),
        }
        .into()
    }

    pub fn tile_bounds(&self) -> Bounds2Di16 {
        Bounds2D::with_size(self.tile, self.size().as_i16())
    }

    pub fn entrance(&self) -> vec2i16 {
        self.tile // TODO
    }

    pub fn id(&self) -> Id{
        self.id
    }
}

/// For Memkeep::insert().
impl SetId for Building {
    fn set_id(&mut self, id: Id) {
        self.id = id;
    }
}

impl Display for Building {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}{}@{}", self.typ, self.id, self.tile)
    }
}
