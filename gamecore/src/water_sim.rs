use crate::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct WaterSim {
    level_at: HashMap<vec2i16, f32>,
    pub velocity_left_of: HashMap<vec2i16, f32>,
    pub velocity_under_of: HashMap<vec2i16, f32>,
}

impl WaterSim {
    pub fn tick(&mut self, tilemap: &Tilemap) {
        self.tick_flow(tilemap);
		//self.tick_DIFFUSION(tilemap);
    }

    pub fn tick_flow(&mut self, tilemap: &Tilemap) {
        // d v left = d p left = p - pleft

        const MU: f32 = 0.03;

        // 🪲 TODO: velocity misses sources to the right (water tile)
        for pos in canal_tiles(tilemap) {
            let level_here = self.water_level_at(tilemap, pos);
            // left/under
            {
                let left = pos + vec2(-1, 0);
                let level_left = self.water_level_at(tilemap, left);
                // TODO: clamp if neighbor empty
                let dvx = if can_flow(tilemap, left) { -MU * (level_here - level_left) } else { 0.0 };
                let v_old = self.velocity_left_of.get(&pos).copied().unwrap_or_default();
                let v_new = v_old + dvx;
                self.velocity_left_of.insert(pos, v_new);
            }

            {
                let under = pos + vec2(0, -1);
                let level_under = self.water_level_at(tilemap, under);
                // TODO: clamp if neighbor empty
                let dvy = if can_flow(tilemap, under) { -MU * (level_here - level_under) } else { 0.0 };
                let v_old = self.velocity_under_of.get(&pos).copied().unwrap_or_default();
                let v_new = v_old + dvy;
                self.velocity_under_of.insert(pos, v_new);
            }

        }

        for pos in canal_tiles(tilemap) {
            let mut level = self.level_at.get(&pos).copied().unwrap_or_default();

			level += self.velocity_left_of(pos) - self.velocity_right_of(pos);
			level += self.velocity_under_of(pos) - self.velocity_upper_of(pos);

			//let level = level.clamp(0.0, 100.0);
            self.level_at.insert(pos, level);
        }
    }

	fn velocity_left_of(&self, pos: vec2i16) -> f32{
		self.velocity_left_of.get(&pos).copied().unwrap_or_default()
	}

	fn velocity_under_of(&self, pos: vec2i16) -> f32{
		self.velocity_under_of.get(&pos).copied().unwrap_or_default()
	}

	fn velocity_right_of(&self, pos: vec2i16) -> f32{
		self.velocity_left_of.get(&(pos + vec2::EX)).copied().unwrap_or_default()
	}

	fn velocity_upper_of(&self, pos: vec2i16) -> f32{
		self.velocity_under_of.get(&(pos + vec2::EY)).copied().unwrap_or_default()
	}

    pub fn tick_DIFFUSION(&mut self, tilemap: &Tilemap) {
        let mut next = HashMap::default();

        for idx in canal_tiles(tilemap) {
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

        self.level_at = next;
    }

    pub fn water_level_at(&self, tilemap: &Tilemap, tile: vec2i16) -> f32 {
        if tilemap.at(tile) == Tile::Water { 100.0 } else { self.level_at.get(&tile).copied().unwrap_or(0.0) }
    }
}

impl Default for WaterSim {
    fn default() -> Self {
        Self {
            level_at: default(),
            velocity_left_of: default(),
            velocity_under_of: default(),
        }
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
