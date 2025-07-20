use crate::prelude::*;

pub struct State {
    pub x: f64,
}

impl State {
    pub fn tick(&mut self) {
        self.x += 2.0;
        if self.x > 100.0 {
            self.x = 0.0
        }
    }

    pub fn render(&self, out: &mut Output) {
        out.debug = "hello".into()
    }
}
