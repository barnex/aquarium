use crate::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct WaterSim {
    pub h: HashMap<vec2i16, f32>,
    pub p: HashMap<vec2i16, vec2f>,
}

impl WaterSim {
    pub fn tick(&mut self, tilemap: &Tilemap) {
        for i in 0..1 {
            self.tick_once(tilemap);
        }
    }

    pub fn tick_once(&mut self, tilemap: &Tilemap) {
        let dt = 0.1;
        let mut delta_h = HashMap::<vec2i16, f32>::default();
        let mut delta_p = HashMap::<vec2i16, vec2f>::default();

        for pos in canal_tiles(tilemap) {
            // 🚰 If directly connected to source (Water tile),
            // set level to maximum.
            let is_source = [[-1, 0], [1, 0], [0, -1], [0, 1]] //_
                .into_iter()
                .map(|[x, y]| pos + vec2(x, y))
                .any(|pos2| tilemap.at(pos2) == Tile::Water);
            if is_source {
                self.h.insert(pos, 1.0);
            }

            // 📏 my water height
            let h1 = self.h.get(&pos).copied().unwrap_or_default();
            let p1 = self.p.get(&pos).copied().unwrap_or_default();

            let neighbors = [[-1, 0], [1, 0], [0, -1], [0, 1]] //_
                .into_iter()
                .map(|[x, y]| pos + vec2(x, y))
                .filter(|pos2| tilemap.at(*pos2) == Tile::Canal);

            for pos2 in neighbors {
                // 📏 neighbor water height
                let h2 = self.h.get(&pos2).copied().unwrap_or_default();

                let (src, dst) = (pos, pos2);
                let to_neighbor = (pos2 - pos).as_f32(); // unit vector

                if h1 > h2 && h1 > 0.0 {
                    let dh = (h1 - h2) * dt; // 💧 transferred amount

                    // transfer matter
                    *delta_h.entry(src).or_default() -= dh;
                    *delta_h.entry(dst).or_default() += dh;

                    // transfer corresponding momentum
                    let fraction = dh / h1; // 💧/💧💧 transferred fraction
                    let momentum_xfer = fraction * p1;
                    *delta_p.entry(src).or_default() -= momentum_xfer;
                    *delta_p.entry(dst).or_default() += momentum_xfer;

                    // generated momentum in destination
                    //*delta_p.entry(dst).or_default() += dh * to_neighbor;
                }

                // let dh = ((h2 - h1).abs().powf(2.0) + (h2 - h1).abs() * f32::min(h1, h2)) * dt;
                // //let dh = (h1 - h2) * dt;
                // let dh = dh.clamp(0.0, f32::max(h1, h2));
                // let dh = if h2 > h1 { 0.0 } else { dh };
                // debug_assert!(dh >= 0.0);

                // *delta_h.entry(src).or_default() -= dh;
                // *delta_h.entry(dst).or_default() += dh;

                // let dp = dt * dh * to_neighbor;
                // *delta_p.entry(dst).or_default() += dp;

                // // propagator
                // // transfer mass
                // let p1 = self.p.get(&pos).copied().unwrap_or_default().dot(to_neighbor);

                // let dh = (p1.abs() * dt).clamp(0.0, h1); // clamp to h[i] *dt?
                // *delta_h.entry(src).or_default() -= dh;
                // *delta_h.entry(dst).or_default() += dh;

                // // transfer momentum: TODO: clamp?
                // let dp = if h1 != 0.0 { p1.signum() * p1.abs() * dh / h1 } else { 0.0 };
                // *delta_p.entry(src).or_default() -= dp * to_neighbor;
                // *delta_p.entry(dst).or_default() += dp * to_neighbor;
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
        Self { h: default(), p: default() }
    }
}

fn can_flow(tilemap: &Tilemap, pos: vec2i16) -> bool {
    match tilemap.at(pos) {
        Tile::Canal | Tile::Water => true,
        _ => false,
    }
}

fn canal_tiles(tilemap: &Tilemap) -> impl Iterator<Item = vec2i16> {
    tilemap.enumerate_all().filter_map(|(tile, mat)| (mat == Tile::Canal).then_some(tile))
}
