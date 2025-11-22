use crate::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct World {
    pub critters: Vec<Critter>,
}

impl World {
    pub fn test() -> Self {
        let critters = vec![Critter::new(4)];

        Self { critters }
    }

    pub fn tick(&mut self, now: f64, dt: f32) {
        self.critters.iter_mut().for_each(|v| v.tick(now, dt));
    }

    pub(crate) fn draw(&self, out: &mut Out) {
        self.draw_background(out);
        self.critters.iter().for_each(|v| v.draw(out));
    }

    fn draw_background(&self, out: &mut Out) {
        let (w, h) = out.viewport_size.as_i32().into();
        let bg = [0, 0, 80];
        out.draw_rect_screen(0, Rectangle::from((((0, 0), (w, h)), bg)).with_fill(bg));
    }
}
