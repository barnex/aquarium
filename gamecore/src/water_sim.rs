use crate::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct WaterSim {}

impl WaterSim {
    pub fn tick(&mut self, tilemap: &Tilemap) {

	}

}

impl Default for WaterSim {
    fn default() -> Self {
        Self {}
    }
}
