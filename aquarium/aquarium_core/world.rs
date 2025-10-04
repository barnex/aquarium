use crate::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct World {
    pub critters: Vec<Critter>,
}
impl World {
    pub(crate) fn test() -> Self {
        Self {
            critters: vec![Critter { head_pos: vec2f(10.0, 20.0) }],
        }
    }

	pub(crate) fn draw(&self, out: &mut Out){
		for crit in &self.critters{
			crit.draw(out);
		}
	}
	
	pub(crate) fn tick(&mut self)  {
		self.critters.iter_mut().for_each(Critter::tick);	
	}
}
