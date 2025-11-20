use crate::prelude::*;

pub struct Brain{
    pub signals: Vec<f32>,
    pub neurons: Vec<Neuron>,
}

pub struct Neuron{
    pub bias: f32,
    pub weights: Vec<(u8, f32)>
}

