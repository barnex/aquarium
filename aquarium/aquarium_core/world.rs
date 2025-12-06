use crate::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct World {
    pub critters: Vec<Critter>,
    pub food: Vec<vec2f>,
}

impl World {
    pub fn test1() -> Self {
        let critters = vec![Critter::new(8, 6)];
        let food = vec![vec2(120.0, 230.0)]; //, vec2(110.0, 55.0), vec2(410.0, 100.0)];

        Self { critters, food }
    }

    /// Test world with a dummy creature that is just a harmonic oscillator,
    /// to test physics.
    pub fn harmonic_osc() -> Self {
        let critters = vec![Critter::harmonic_osc()];
        let food = vec![];

        Self { critters, food }
    }

    pub fn tick(&mut self, now: f64, dt: f32) {
        self.critters.iter_mut().for_each(|v| v.tick(now, dt, &self.food));
    }

    pub(crate) fn draw(&self, out: &mut Out) {
        self.draw_background(out);
        self.critters.iter().for_each(|v| v.draw(out));
        self.food.iter().for_each(|v| self.draw_food(out, *v));
    }

    fn draw_food(&self, out: &mut Out, pos: vec2f) {
        let color = RGBA::GREEN;
        let radius = vec2(2, 2);
        out.draw_rect_screen(L_SPRITES + 1, Rectangle::with_radius(pos.as_i32(), radius, color));
    }

    fn draw_background(&self, out: &mut Out) {
        let (w, h) = out.viewport_size.as_i32().into();
        let bg = [0, 0, 80];
        out.draw_rect_screen(0, Rectangle::from((((0, 0), (w, h)), bg)).with_fill(bg));
    }
}
