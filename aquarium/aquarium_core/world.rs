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

    pub(crate) fn draw(&self, out: &mut Out) {

        let (w, h) = out.viewport_size.as_i32().into();
        let bg = [0, 0, 80];
        out.draw_rect_screen(0, Rectangle::from((((0, 0), (w, h)), bg)).with_fill(bg));

        for crit in &self.critters {
            crit.draw(out);
        }
    }

    pub(crate) fn tick(&mut self) {
        self.critters.iter_mut().for_each(Critter::tick);
    }
}
