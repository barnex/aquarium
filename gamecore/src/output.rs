use crate::prelude::*;

pub struct Output {
    pub debug: String,
}

impl Output {
    pub fn new() -> Self {
        Self { debug: default() }
    }

    pub fn clear(&mut self)  {
        self.debug.clear();
    }
}
