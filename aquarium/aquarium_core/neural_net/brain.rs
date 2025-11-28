use std::iter::zip;

use crate::prelude::*;

/*
        --- --- --- --- --- --- --- --- ---
       | v | v | v | v | v | v | v | v | v |
        --- --- --- --- --- --- --- --- ---
       |   |   |   |   |   |   |   |   |   |
        --- --- --- --- --- --- --- --- ---
       | l |   |   |   |   |   |   |   | r |
        --- --- --- --- --- --- --- --- ---
       | l |   |   |   |   |   |   |   | r |
        --- --- --- --- --- --- --- --- ---
       | l |   |   | L | R |   |   |   | r |
        --- --- --- --- --- --- --- --- ---
       |   |   |   | L | R |   |   |   |   |
        --- --- --- --- --- --- --- --- ---
       |   |   |   | L | R |   |   |   |   |
        --- --- --- --- --- --- --- --- ---

*/

#[derive(Serialize, Deserialize)]
pub struct Brain {
    pub inputs: Vec2D<f32>,
    pub signals: Vec2D<f32>,
    pub sigbuf: Vec2D<f32>,
    pub neurons: Vec2D<Neuron>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Neuron {
    pub bias: f32,
    pub weights: Vec<(u8, f32)>,
}

impl Brain {
    pub fn new(size: impl Into<vec2u>) -> Self {
        let size = size.into();
        let signals = Vec2D::new(size);
        let inputs = signals.clone();
        let sigbuf = signals.clone();
        let neurons = Vec2D::new(size);
        Self { inputs, signals, sigbuf, neurons }
    }

    pub fn size(&self) -> vec2u {
        self.signals.size()
    }

    pub fn update(&mut self) {
        let inputs = &self.inputs.values;
        let signals = &self.signals.values;
        let sigbuf = &mut self.sigbuf.values;
        let neurons = &self.neurons.values;

        assert!(signals.len() == sigbuf.len());
        assert!(signals.len() == neurons.len());
        assert!(sigbuf.len() == neurons.len());
        assert!(sigbuf.len() == inputs.len());

        for i in 0..sigbuf.len() {
            sigbuf[i] = inputs[i] + neurons[i].bias;
            let weights = &neurons[i].weights;
            for (k, w) in weights.iter().copied() {
                sigbuf[i] += w * signals[k as usize];
            }
        }

        let signals = &mut self.signals.values;
        // Activation function: rectifier + clamp
        // + copy to signals
        for i in 0..sigbuf.len() {
            signals[i] = sigbuf[i].clamp(0.0, 1.0);
        }
    }

    pub fn draw(&self, out: &mut Out) {
        let l = L_SPRITES + 1;
        //out.draw_text(l, (0, 0), format!("brain {}", self.size()));

        let stroke = RGBA::DARK_GRAY;

        for (idx, v) in self.signals.enumerate() {
            let pos = self.neuron_to_screen_pos(idx);
            let size = Self::NEURON_SCREEN_SIZE + 1;
            let fill = RGBA(colormap(v).into());
            out.draw_rect_screen(l, Rectangle::with_size(pos, size, stroke).with_fill(fill));
        }

        for (idx, neuron) in self.neurons.enumerate_ref() {
            let start = self.neuron_to_screen_pos(idx);
            for (i, w) in &neuron.weights {
                let idx = self.neurons.reverse_index(i.as_());
                let end = self.neuron_to_screen_pos(idx);
                let off = Self::NEURON_SCREEN_SIZE / 2;
                out.draw_line_screen(L_SPRITES + 2, Line::new(start, end).translated(off).with_color(RGBA([255, 255, 255, 128])));
            }
        }
    }

    pub fn neuron_to_screen_pos(&self, idx: vec2u) -> vec2i {
        idx.as_i32() * Self::NEURON_SCREEN_SIZE + Self::BRAIN_SCREEN_OFFSET
    }

    pub fn screen_pos_to_neuron(&self, screen: vec2i) -> Option<vec2u> {
        let idx = (screen - Self::BRAIN_SCREEN_OFFSET) / Self::NEURON_SCREEN_SIZE;
        self.signals.in_bounds(idx).then_some(idx.as_u32())
    }

    const NEURON_SCREEN_SIZE: vec2i = vec2i(12, 12);
    const BRAIN_SCREEN_OFFSET: vec2i = vec2i(10, 10);
}

fn colormap(v: f32) -> vec4u8 {
    let s = 2.0;

    let values = [
        -1.0 * s, //_
        -0.5 * s,
        0.0,
        0.5 * s,
        1.0 * s,
    ];

    let colors = [
        RGB::CYAN, //_
        RGB::BLUE,
        RGB::BLACK,
        RGB::RED,
        RGB::YELLOW,
    ]
    .map(|v| vec3::from(v.0).as_f32().append(255.0));

    for ((v1, c1), (v2, c2)) in zip(values, colors).tuple_windows() {
        if (v1..=v2).contains(&v) {
            return linterp(v1, c1, v2, c2, v).map(|v| v.clamp(0.0, 255.0) as u8);
        }
    }
    vec4(128, 128, 128, 255)
}

pub fn zap(brain: &mut Brain, screen_pos: Option<vec2i>) {
    if let Some(screen_pos) = screen_pos {
        if let Some(neuron) = brain.screen_pos_to_neuron(screen_pos) {
            brain.inputs.set(neuron, 1.0);
        }
    }
}
