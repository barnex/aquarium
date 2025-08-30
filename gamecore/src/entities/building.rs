use crate::prelude::*;

const MAX_RES_SLOTS: usize = 4;

#[derive(Serialize, Deserialize, Debug)]
pub struct Building {
    pub id: Id,
    pub typ: BuildingTyp,
    pub tile: vec2i16,
    pub workers: CSet<Id>,
    pub downstream: CSet<Id>,
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
            downstream: default(),
        }
    }

    pub fn processes_resource(&self, res: ResourceTyp) -> bool {
        Self::resource_indices(self.typ)[res as usize].is_some()
    }

    pub fn add_resource(&self, res: ResourceTyp) -> Status {
        let (i, cap) = Self::resource_indices(self.typ)[res as usize]?;
        if self.resources[i].get() < cap {
            self.resources[i].inc(1);
            OK
        } else {
            FAIL
        }
    }

    pub fn iter_resources(&self) -> impl Iterator<Item = (ResourceTyp, u16)> {
        // TODO: reverse index instead of linear search
        Self::resource_indices(self.typ)
            .into_iter()
            .enumerate()
            .filter_map(|(r, v)| v.map(|(i, _cap)| (r, i)))
            .map(|(r, i)| (ResourceTyp::try_from_primitive(r as u8).unwrap(), i))
            .map(|(r, i)| (r, self.resources[i as usize].get()))
    }

    /// Index in Building.resrouces and max capacity.
    fn resource_indices(typ: BuildingTyp) -> [Option<(usize, u16)>; ResourceTyp::COUNT] {
        match typ {
            BuildingTyp::HQ => [None, Some((0, 1000)), Some((1, 1000))],
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
