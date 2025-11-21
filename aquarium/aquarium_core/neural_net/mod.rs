use crate::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct Brain {
    pub size: vec2u,
    pub signals: Vec<f32>,
    pub neurons: Vec<Neuron>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Neuron {
    pub bias: f32,
    pub weights: Vec<(u8, f32)>,
}

impl Brain {
    pub fn new(size: impl Into<vec2u>) -> Self {
        let size = size.into();
        let n = size.product() as usize;
        let signals = vec![0.0; n];
        let neurons = vec![default(); n];
        Self { size, signals, neurons }
    }

    pub fn draw(&self, out: &mut Out) {
        out.draw_text(L_SPRITES + 1, (0, 0), "brain");
    }
}
