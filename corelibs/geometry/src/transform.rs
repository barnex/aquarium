use matrix::*;
use serde::{Deserialize, Serialize};
use vector::*;

/// Translation + Scale.
/// TODO: 💀 currently only used for `Prop`s. Remove?
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Transform {
    pub translation: vec3f,
    pub scale: f32,
}

impl Transform {
    pub fn translation(translation: vec3f) -> Self {
        Self { translation, ..Default::default() }
    }

    pub fn matrix(&self) -> mat4x4f {
        translation_matrix(self.translation) * scale_matrix(self.scale)
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self { translation: Default::default(), scale: 1.0 }
    }
}
