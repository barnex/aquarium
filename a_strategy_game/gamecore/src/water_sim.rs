use crate::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct WaterSim {
    pub h: HashMap<vec2i16, f32>,
    pub p: HashMap<vec2i16, vec2f>,
    pub farm_water: HashMap<vec2i16, f32>,
}

impl WaterSim {
    pub fn major_tick(&mut self, tilemap: &Tilemap) {
        // ☘️ irrigate farm land
        for (&pos, h) in self.h.iter_mut() {
            // 🚰 If directly connected to source (Water tile),
            // set level to maximum.
            for neigh in [[-1, 0], [1, 0], [0, -1], [0, 1]] //_
                .into_iter()
                .map(|[x, y]| pos + vec2(x, y))
            {
                if tilemap.at(neigh) == Tile::Farmland {
                    if *h > 0.05 {
                        *self.farm_water.entry(neigh).or_default() += *h;
                        *h *= 0.99;
                        *self.p.entry(pos).or_default() *= 0.99;
                    }
                }
            }
        }

        /*
        for pos in farmland_tiles(tilemap) {
            const MAX_WETNESS: f32 = 0.3;
            let h = self.h.entry(pos).or_default();
            *h *= 0.997; // slowly dry out
            let h = *h;

            *self.p.entry(pos).or_default() = vec2::ZERO;

            if h < MAX_WETNESS {
                for neigbor in [[-1, 0], [1, 0], [0, -1], [0, 1]] //_
                    .into_iter()
                    .map(|[x, y]| pos + vec2(x, y))
                {
                    if let Some(h2) = self.h.get(&neigbor) {
                        let dh = (h - h2) * dt * 0.05; // 👈 spill rate

                        // also take away momentum
                        if h > 0.0 {
                            let p2 = self.p.get(&neigbor).copied().unwrap_or_default();
                            let fraction = dh / h; // 💧/💧💧 transferred fraction
                            let momentum_xfer = fraction * p2;
                            *delta_p.entry(neigbor).or_default() -= momentum_xfer;
                        }

                        *delta_h.entry(neigbor).or_default() += dh;
                        *delta_h.entry(pos).or_default() -= dh;
                    }
                }
            }
        }
        */
    }

    fn tick_sources(&mut self, tilemap: &Tilemap) {
        for (&pos, h) in self.h.iter_mut() {
            // 🚰 If directly connected to source (Water tile),
            // set level to maximum.
            let is_source = [[-1, 0], [1, 0], [0, -1], [0, 1]] //_
                .into_iter()
                .map(|[x, y]| pos + vec2(x, y))
                .any(|pos2| tilemap.at(pos2) == Tile::Water);
            if is_source {
                *h = 1.0;
            }
        }
    }

    pub fn minor_tick(&mut self, tilemap: &Tilemap) {
        self.tick_sources(tilemap);
        self.tick_flow(tilemap);
    }

    pub fn tick_flow(&mut self, tilemap: &Tilemap) {
        let dt = 0.1;
        let mut delta_h = HashMap::<vec2i16, f32>::default();
        let mut delta_p = HashMap::<vec2i16, vec2f>::default();

        for pos in self.h.keys().copied() {
            // 📏 my water height
            let h1 = self.h.get(&pos).copied().unwrap_or_default();
            let p1 = self.p.get(&pos).copied().unwrap_or_default();

            let neighbors = [[-1, 0], [1, 0], [0, -1], [0, 1]] //_
                .into_iter()
                .map(|[x, y]| pos + vec2(x, y))
                .filter(|pos2| tilemap.at(*pos2) == Tile::Canal);

            // propagator
            if h1 > 0.0 {
                let p = p1.x();
                if p > 0.0 {
                    let dst = pos + vec2::EX;
                    if tilemap.at(dst) == Tile::Canal {
                        let dh = p * dt;

                        let fraction = dh / h1; // 💧/💧💧 transferred fraction
                        let momentum_xfer = fraction * p1;
                        *delta_p.entry(pos).or_default() -= momentum_xfer;
                        *delta_p.entry(dst).or_default() += momentum_xfer;

                        *delta_h.entry(pos).or_default() -= dh;
                        *delta_h.entry(dst).or_default() += dh;
                    } else {
                        self.p.get_mut(&pos).unwrap()[0] *= 0.5;
                    }
                }

                if p < 0.0 {
                    let dst = pos - vec2::EX;
                    if tilemap.at(dst) == Tile::Canal {
                        let dh = p.abs() * dt;

                        let fraction = dh / h1; // 💧/💧💧 transferred fraction
                        let momentum_xfer = fraction * p1;
                        *delta_p.entry(pos).or_default() -= momentum_xfer;
                        *delta_p.entry(dst).or_default() += momentum_xfer;

                        *delta_h.entry(pos).or_default() -= dh;
                        *delta_h.entry(dst).or_default() += dh;
                    } else {
                        self.p.get_mut(&pos).unwrap()[0] *= 0.5;
                    }
                }

                let p = p1.y();
                if p > 0.0 {
                    let dst = pos + vec2::EY;
                    if tilemap.at(dst) == Tile::Canal {
                        let dh = p * dt;

                        let fraction = dh / h1; // 💧/💧💧 transferred fraction
                        let momentum_xfer = fraction * p1;
                        *delta_p.entry(pos).or_default() -= momentum_xfer;
                        *delta_p.entry(dst).or_default() += momentum_xfer;

                        *delta_h.entry(pos).or_default() -= dh;
                        *delta_h.entry(dst).or_default() += dh;
                    } else {
                        self.p.get_mut(&pos).unwrap()[1] *= 0.5;
                    }
                }

                if p < 0.0 {
                    let dst = pos - vec2::EY;
                    if tilemap.at(dst) == Tile::Canal {
                        let dh = p.abs() * dt;

                        let fraction = dh / h1; // 💧/💧💧 transferred fraction
                        let momentum_xfer = fraction * p1;
                        *delta_p.entry(pos).or_default() -= momentum_xfer;
                        *delta_p.entry(dst).or_default() += momentum_xfer;

                        *delta_h.entry(pos).or_default() -= dh;
                        *delta_h.entry(dst).or_default() += dh;
                    } else {
                        self.p.get_mut(&pos).unwrap()[1] *= 0.5;
                    }
                }
            }

            for pos2 in neighbors {
                // 📏 neighbor water height
                let h2 = self.h.get(&pos2).copied().unwrap_or_default();

                let (src, dst) = (pos, pos2);
                let to_neighbor = (pos2 - pos).as_f32(); // unit vector

                if h1 > h2 && h1 > 0.0 {
                    // TODO ENABLE !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
                    // diffuse matter
                    let dh = (h1 - h2) * dt; // 💧 transferred amount
                    *delta_h.entry(src).or_default() -= dh;
                    *delta_h.entry(dst).or_default() += dh;

                    // transfer corresponding momentum
                    let fraction = dh / h1; // 💧/💧💧 transferred fraction
                    let momentum_xfer = fraction * p1;
                    *delta_p.entry(src).or_default() -= momentum_xfer;
                    *delta_p.entry(dst).or_default() += momentum_xfer;

                    // generated momentum in destination
                    *delta_p.entry(dst).or_default() += dh * to_neighbor;
                }
            }
        }

        // apply deltas
        for (&pos, &delta_h) in &delta_h {
            *self.h.entry(pos).or_default() += delta_h;
        }
        for (&pos, &delta_p) in &delta_p {
            let p = self.p.entry(pos).or_default();
            *p += delta_p;
            //*p = (*p).map(|v|v.clamp(-1.0, 1.0));
        }

        // remove orphan water
        let orphans = self.h.keys().chain(self.p.keys()).copied().filter(|&tile| !tilemap.at(tile).can_have_water()).collect_vec();
        for tile in orphans {
            self.h.remove(&tile);
            self.p.remove(&tile);
        }
    }

    pub fn water_level_at(&self, tile: vec2i16) -> f32 {
        self.h.get(&tile).copied().unwrap_or_default()
    }

    pub fn water_speed_at(&self, tile: vec2i16) -> vec2f {
        self.p.get(&tile).copied().unwrap_or_default()
    }
}

impl Default for WaterSim {
    fn default() -> Self {
        Self {
            h: default(),
            p: default(),
            farm_water: default(),
        }
    }
}

fn can_flow(tilemap: &Tilemap, pos: vec2i16) -> bool {
    match tilemap.at(pos) {
        Tile::Canal | Tile::Water => true,
        _ => false,
    }
}
