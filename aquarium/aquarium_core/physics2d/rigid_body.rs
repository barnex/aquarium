use crate::prelude::*;
use core_util::With as _;
use matrix::*;

// comment
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RigidBody {
    pub mass: f32,
    pub position: vec2f,

    pub velocity_half: vec2f,
    pub velocity: vec2f,

    pub acceleration: vec2f,
    pub force: vec2f,

    pub rot_inertia: f32,
    pub rotation: f32,
    pub rot_velocity_half: f32,
    pub rot_velocity: f32,
    pub rot_accel: f32,
    pub torque: f32,
}

impl RigidBody {
    pub fn new(position: vec2f, mass: f32, rot_inertia: f32) -> Self {
        debug_assert!(mass > 0.0);
        debug_assert!(rot_inertia > 0.0);

        Self {
            mass,
            position,
            velocity_half: default(),
            velocity: default(),
            acceleration: default(),
            force: default(),
            rot_inertia,
            rotation: default(),
            rot_velocity_half: default(),
            rot_velocity: default(),
            rot_accel: default(),
            torque: default(),
        }
    }

    pub fn new_at_origin(mass: f32, rot_inertia: f32) -> Self {
        Self::new(vec2::ZERO, mass, rot_inertia)
    }

    pub fn update_accel(&mut self) {
        self.acceleration = self.force / self.mass;
        self.rot_accel = self.torque / self.rot_inertia;
    }

    pub(crate) fn update_velocity_verlet(&mut self, dt: f32) {
        self.velocity = self.velocity_half + dt / 2.0 * self.acceleration;
        self.rot_velocity = self.rot_velocity_half + dt / 2.0 * self.rot_accel;
        self.velocity_half = self.velocity + dt / 2.0 * self.acceleration;
        self.rot_velocity_half = self.rot_velocity + dt / 2.0 * self.rot_accel;
    }

    pub fn update_position_verlet(&mut self, dt: f32) {
        self.position += dt * self.velocity_half;
        self.rotation += dt * self.rot_velocity_half;
        self.rotation = wrap_angle(self.rotation);
    }

    //fn tick_euler(&mut self, dt: f32) {
    //    self.update_accel();
    //    self.update_velocity_euler(dt);
    //    self.update_position_euler(dt);
    //}

    pub(crate) fn update_velocity_euler(&mut self, dt: f32) {
        self.velocity += dt * self.acceleration;
        self.rot_velocity += dt * self.rot_accel;
    }

    pub(crate) fn update_position_euler(&mut self, dt: f32) {
        self.position += dt * self.velocity;
        self.rotation += dt * self.rot_velocity;
        self.rotation = wrap_angle(self.rotation);
    }

    pub fn dampen_position(&mut self, dt: f32) {
        self.velocity = self.acceleration;
        self.position += dt * self.velocity;
        self.rotation += dt * self.rot_accel;
        self.rotation = wrap_angle(self.rotation);
    }

    pub fn tick_old(&mut self, dt: f32, force: vec2f, torque: f32, can_walk: impl Fn(vec2f) -> bool) {
        // translation
        self.acceleration = force / self.mass;
        self.velocity += dt * force / self.mass;

        //self.set_velocity((1.0 - dt * gs.linear_damping()) * self.velocity);
        let delta_pos = dt * self.velocity;
        let new_pos = self.position + delta_pos;
        if can_walk(new_pos) {
            // 👈 hack for infite sized tile map
            // no collision
            self.position = new_pos;
        } else {
            // collision: bounce along normal direction, keep moving along tangent:
            for i in 0..2 {
                // x, y
                let new_pos = self.position + delta_pos.with(|v| v[1 - i] = 0.0);
                if can_walk(new_pos) {
                    self.position = new_pos;
                } else {
                    self.velocity = self.velocity.with(|v| v[i] = -0.5 * v[i]);
                }
            }
        }

        // rotation
        let mut theta = self.rotation;
        theta += dt * self.rot_velocity;
        if theta > PI {
            theta -= 2.0 * PI;
        } else if theta < -PI {
            theta += 2.0 * PI;
        }
        self.rotation = theta;

        const MAX_ROT: f32 = 10.0;
        self.rot_accel = torque / self.rot_inertia;
        self.rot_velocity += dt * self.rot_accel;
    }

    pub fn inverse_rotation_matrix(&self) -> mat2x2f {
        let (sin, cos) = f32::sin_cos(self.rotation);
        mat2x2f::from([[cos, sin], [-sin, cos]])
    }

    pub fn rotation_matrix(&self) -> mat2x2f {
        let (sin, cos) = f32::sin_cos(self.rotation);
        mat2x2f::from([[cos, -sin], [sin, cos]])
    }

    pub fn direction(&self) -> vec2f {
        let (sin, cos) = f32::sin_cos(self.rotation);
        vec2(cos, -sin)
    }

    pub fn transform_rel_pos(&self, rel_pos: vec2f) -> vec2f {
        (self.rotation_matrix() * rel_pos) + self.position
    }

    pub fn transform_vector(&self, vector: vec2f) -> vec2f {
        self.rotation_matrix() * vector
    }

    pub fn velocity_of_rel_pos(&self, rel_pos: vec2f) -> vec2f {
        let (x, y) = rel_pos.into();
        let rot_vel = 2.0 * PI * self.rot_velocity * vec2(-y, x);
        rot_vel + self.velocity
    }

    pub fn transform_rotation(&self, rotation: f32) -> f32 {
        self.rotation + rotation
    }

    pub fn transform_frame(&self, (pos, rot): (vec2f, f32)) -> (vec2f, f32) {
        (self.transform_rel_pos(pos), self.transform_rotation(rot))
    }
}

fn wrap_angle(theta: f32) -> f32 {
    let mut theta = theta;
    if theta > PI {
        theta -= 2.0 * PI;
    } else if theta < -PI {
        theta += 2.0 * PI;
    }
    theta
}

fn draw_body(out: &mut Out, body: &RigidBody) {
    let pos = body.position.as_i32();
    let color = RGBA::WHITE;

    // draw center
    let s = vec2(2, 2);
    out.draw_rect_screen(L_SPRITES, Rectangle::new((pos - s, pos + s), color));

    // draw frame/axes
    let ax_len = 15.0;
    let x = body.transform_rel_pos(vec2::EX * ax_len).as_i32();
    let y = body.transform_rel_pos(vec2::EY * ax_len).as_i32();
    out.draw_line_screen(L_SPRITES, Line::new(pos, x).with_color(color));
    out.draw_line_screen(L_SPRITES, Line::new(pos, y).with_color(color));
}
