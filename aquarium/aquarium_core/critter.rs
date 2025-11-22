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
    pub fn new(len: usize) -> Self {
        let mut brain = Brain::new([8, (len + 5) as u32]);

        let mut rng = ChaCha8Rng::seed_from_u64(123);
        //brain.signals.iter_mut().for_each(|v| *v = rng.gen_range(-2.0..=2.0));

        Self {
            body: Contraption::rope(len),
            brain,
            crawl_amplitude: 0.2,
            crawl_frequency: -0.3,
            crawl_wavenumber: 0.8,
            crawl_gamma: 1.0,
        }
    }

    pub fn tick(&mut self, t: f64, dt: f32, food: &[vec2f]) {
        self.brain.update(); // <<< TODO: don't overwrite input neurons, annoying for visualization
        self.update_vision(food);
        self.update_body_sense();
        self.tick_crawl_test(t);
        self.body.tick(dt);
        // brain & sensory
    }

    fn update_body_sense(&mut self) {
        let bones = &self.body.bones;
        let brain = &mut self.brain.signals;

        let y0 = brain.size().y() as usize - bones.len();
        let ix1 = 0;
        let ix2 = (brain.size().x() - 1).as_();

        for ((i1, b1), (_, b2)) in bones.iter().enumerate().tuple_windows() {
            let angle = b1.direction().cross(b2.direction());
            let y = y0 + i1;
            brain.set(vec2(ix1, y).as_u32(), 7.0 * angle);
            brain.set(vec2(ix2, y).as_u32(), -7.0 * angle);
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

        let n = (self.brain.size().x() - 1) as f32;
        let eta = 1.0 / n;

        for &food in food {
            let dir = (food - or).normalized();
            let dist = (food - or).len();
            let x = dir.dot(ex); //(-1..1)
            let y = dir.dot(ey);
            if y > 0.0 {
                // only see before you
                let i = linterp(-1.0, 0.0 + eta / 2.0, 1.0, n - eta / 2.0, x).clamp(0.0, n) as u32;
                let sig = (1000.0 / dist).clamp(0.0, 1.0);
                let sig = match sig.is_finite() {
                    true => sig,
                    false => 0.0,
                };
                self.brain.signals.set(vec2(i, 0), sig);
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
            spring.sin_angle = a;
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
