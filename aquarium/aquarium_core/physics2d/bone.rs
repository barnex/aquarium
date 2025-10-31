use crate::prelude::*;

#[derive(Serialize, Deserialize, Clone)]
pub struct Bone {
    pub body: RigidBody,
    pub len: f32,
}

impl Bone {
    pub fn new(mass: f32, rot_inertia: f32, len: f32) -> Self {
        debug_assert!(len > 0.0);
        Self { body: RigidBody::new(mass, rot_inertia), len }
    }

}
