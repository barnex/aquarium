use crate::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct Spring {
    pub ia: usize,
    pub ib: usize,
    pub anchor_a: vec2f,
    pub anchor_b: vec2f,
    pub k: f32,
    pub sin_angle: f32,
}
