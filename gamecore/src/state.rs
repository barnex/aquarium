use crate::prelude::*;

pub struct State {
    pub inputs: Inputs,

    pub frame: u64,
    pub score: u64,
    pub x: f64,
}

impl State {
    pub fn new() -> Self {
        Self { inputs: default(), frame: 0, x: 0.0, score: default() }
    }

    pub fn tick(&mut self) {
        
        if self.inputs.just_pressed(Button(str16!("b"))){
            self.score+=1
        }
        

        self.frame += 1;
        self.x += 0.5;
        if self.x > 100.0 {
            self.x = 0.0
        }
    }

    pub fn render(&self, out: &mut Output) {
        writeln!(&mut out.debug, "frame {}", self.frame).unwrap();
        writeln!(&mut out.debug, "score {}", self.score).unwrap();
        writeln!(&mut out.debug, "inputs {:?}", self.inputs).unwrap();
    }
}
