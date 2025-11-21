use std::{iter::zip, ops::RangeBounds};

use crate::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct Brain {
    pub signals: Vec2D<f32>,
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
        let neurons = Vec2D::new(size);
        Self { signals, neurons }
    }

    pub fn size(&self) -> vec2u {
        self.signals.size()
    }

    pub fn draw(&self, out: &mut Out) {
        let l = L_SPRITES + 1;
        //out.draw_text(l, (0, 0), format!("brain {}", self.size()));

        let offset = vec2i(10, 10);
        let stroke = RGBA::WHITE;
        let stride = vec2i(8, 8);

        let mut fill_rect = |pos, fill| out.draw_rect_screen(l, Rectangle::with_size(pos, stride + 1, stroke).with_fill(fill).translated(offset));

        for (idx, v) in self.signals.enumerate() {
            let pos = idx.as_i32() * stride;
            fill_rect(pos, RGBA(colormap(v).into()));
        }
    }
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
