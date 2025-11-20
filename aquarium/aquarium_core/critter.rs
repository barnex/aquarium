use crate::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct Critter {
    pub body: Contraption,

    pub crawl_amplitude: f32,
    pub crawl_wavenumber: f32,
    pub crawl_frequency: f32,
    pub crawl_gamma: f32,
}

impl Critter {
    pub fn new(len: usize) -> Self {
        Self {
            body: Contraption::rope(len),
            crawl_amplitude: 0.2,
            crawl_frequency: 0.3,
            crawl_wavenumber: 0.8,
            crawl_gamma: 1.0,
        }
    }

    pub fn tick(&mut self, t: f64, dt: f32) {
        self.tick_crawl_test(t);
        self.body.tick(dt);
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
    }
}
