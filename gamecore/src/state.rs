use std::fmt::Write as _;

use crate::prelude::*;

pub struct State {
    pub frame: u64,
    pub x: f64,
}

impl State {
    pub fn new() -> Self {
        Self { frame: 0, x: 0.0 }
    }

    pub fn tick(&mut self) {
        self.frame += 1;
        self.x += 0.5;
        if self.x > 100.0 {
            self.x = 0.0
        }
    }

    pub fn render(&self, out: &mut Output) {
        writeln!(&mut out.debug, "frame {}", self.frame).unwrap();
    }
}
