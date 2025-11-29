use crate::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct Critter {
    pub body: Contraption,
    pub brain: Brain,

    pub crawl_amplitude: f32,
    pub crawl_wavenumber: f32,
    pub crawl_frequency: f32,
    pub crawl_gamma: f32,
}

impl Critter {
    pub fn new(len: usize, retina: u32) -> Self {
        let mut brain = Brain::new([retina, (len + 1) as u32]);

        //let mut rng = ChaCha8Rng::seed_from_u64(123);
        //brain.signals.iter_mut().for_each(|v| *v = rng.gen_range(-2.0..=2.0));

        // brain.neurons.at_mut(vec2(2, 3)).bias = -0.5;

        //brain.neurons.at_mut(vec2(3, 8)).weights.push((0, 0.8));
        //brain.neurons.at_mut(vec2(3, 6)).weights.push((1, 0.8));
        //brain.neurons.at_mut(vec2(3, 7)).weights.push((3, 0.8));

        //brain.neurons.at_mut(vec2(2, 6)).weights.push((2, 0.8));
        //brain.neurons.at_mut(vec2(2, 7)).weights.push((4, 0.8));
        //brain.neurons.at_mut(vec2(2, 8)).weights.push((5, 0.8));

        Self {
            body: Contraption::rope(len),
            brain,
            crawl_amplitude: 0.2,
            crawl_frequency: -0.3,
            crawl_wavenumber: 0.8,
            crawl_gamma: 1.0,
        }
    }

    pub fn harmonic_osc() -> Self {
        let brain = Brain::new(dbg!((5, 6)));

        Self {
            body: Contraption::harmonic_osc(),
            brain,
            crawl_amplitude: 0.0,
            crawl_frequency: 0.0,
            crawl_wavenumber: 0.0,
            crawl_gamma: 0.0,
        }
    }

    pub fn tick(&mut self, t: f64, dt: f32, food: &[vec2f]) {
        self.update_body_sense();
        self.update_vision(food);
        self.brain.update();

        self.brain_controls_motion();
        self.body.tick(dt);
    }

    fn update_body_sense(&mut self) {
        let bones = &self.body.bones;
        let inputs = &mut self.brain.inputs;

        let ix1 = 0;
        let ix2 = (inputs.size().x() - 1).as_();

        let y0 = inputs.size().y() as usize - bones.len();

        for ((i1, b1), (_, b2)) in bones.iter().enumerate().tuple_windows() {
            let angle = b1.direction().cross(b2.direction());
            let y = y0 + i1;
            inputs.set(vec2(ix1, y).as_u32(), 7.0 * angle);
            inputs.set(vec2(ix2, y).as_u32(), -7.0 * angle);
        }
    }

    fn brain_controls_motion(&mut self) {
        let bones = &mut self.body.bones;
        let brain = &self.brain.signals;

        let y0 = brain.size().y() - bones.len() as u32 + 1;
        let ix1 = brain.size().x() / 2 - 1;
        let ix2 = brain.size().x() / 2;

        for (i, spring) in self.body.springs.iter_mut().enumerate() {
            let y = y0 + i as u32;
            spring.angle_setpoint = (brain.at(vec2(ix1, y)) - brain.at(vec2(ix2, y))) * 0.4 // <<< !!!
        }
    }

    /// Set signals of vision neurons (layer 0) to see food.
    fn update_vision(&mut self, food: &[vec2f]) {
        let head = &self.body.bones[0];
        let matrix = head.rotation_matrix();
        // Y = view direction
        let ey = vec2::from(matrix[0]);
        // X = sideways (retina) direction
        let ex = vec2::from(matrix[1]);
        let or = head.position.as_f32();

        let w = self.brain.size().x();
        let n = (w) as f32;
        let eta = 1.0 / n;

        let brain = &mut self.brain.inputs;

        let iy = 0; // visual input layer

        // reset to 0 first
        for ix in 0..w {
            brain.set(vec2(ix, iy), 0.0);
        }
        // then add food sigals
        for &food in food {
            let dir = (food - or).normalized();
            let dist = (food - or).len();
            let x = dir.dot(ex); //(-1..1)
            let y = dir.dot(ey);
            if y > 0.0 {
                // only see before you
                let ix = linterp(-1.0, 0.0 + eta / 2.0, 1.0, n - eta / 2.0, x).clamp(0.0, n - 1.0) as u32;
                let sig = (1000.0 / dist).clamp(0.0, 1.0);
                let sig = match sig.is_finite() {
                    true => sig,
                    false => 0.0,
                };
                brain.set(vec2(ix, iy), sig);
            }
        }
    }

    fn tick_crawl_test(&mut self, t: f64) {
        let t = t as f32;
        for (i, spring) in self.body.springs.iter_mut().enumerate() {
            let x = i as f32;
            let a = f32::sin(2.0 * PI * t * self.crawl_frequency + x * self.crawl_wavenumber);
            let a = a.abs().powf(self.crawl_gamma) * a.signum();
            let a = self.crawl_amplitude * a;
            spring.angle_setpoint = a;
        }
    }

    pub fn draw(&self, out: &mut Out) {
        self.body.draw(out);
        self.draw_vision(out);
    }

    fn draw_vision(&self, out: &mut Out) {
        let head = &self.body.bones[0];
        let matrix = head.rotation_matrix();
        let ey = vec2::from(matrix[0]);
        let ex = vec2::from(matrix[1]);

        let start = head.position.as_f32();
        let len = 100.0;
        let forward = start + len * ey;
        let sideways = start + len * ex;
        out.draw_line_screen(L_SPRITES + 2, Line::new(start.as_(), forward.as_()).with_color(RGBA::WHITE));
        out.draw_line_screen(L_SPRITES + 2, Line::new(start.as_(), sideways.as_()).with_color(RGBA::YELLOW));
    }
}
