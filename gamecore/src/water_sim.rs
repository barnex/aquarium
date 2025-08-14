use crate::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct WaterSim {
    level_at: HashMap<vec2i16, f32>,
}

impl WaterSim {
    pub fn tick(&mut self, tilemap: &Tilemap) {
        let mut next = HashMap::default();

        for (idx, tile) in tilemap.enumerate_all() {
            if tile == Tile::Canal {
                let mut level = self.water_level_at(tilemap, idx);
                let mut n = 1;

                for [dx, dy] in [[-1, 0], [0, -1], [1, 0], [0, 1]] {
                    let neigh = idx + vec2(dx, dy);
                    let neigh = tilemap.at(neigh);
                    if neigh == Tile::Canal || neigh == Tile::Water {
                        level += self.water_level_at(tilemap, idx + vec2(dx, dy));
                        n += 1;
                    }
                }

                next.insert(idx, level / (n as f32));
            }
        }

        self.level_at = next;
    }

    pub fn water_level_at(&self, tilemap: &Tilemap, tile: vec2i16) -> f32 {
        if tilemap.at(tile) == Tile::Water { 100.0 } else { self.level_at.get(&tile).copied().unwrap_or(0.0) }
    }
}

impl Default for WaterSim {
    fn default() -> Self {
        Self { level_at: default() }
    }
}
