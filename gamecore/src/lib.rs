pub struct State {
    pub x: f64,
}

impl State {
    pub fn tick(&mut self) {
        self.x += 1.0;
        if self.x > 100.0 {
            self.x = 0.0
        }
    }
}
