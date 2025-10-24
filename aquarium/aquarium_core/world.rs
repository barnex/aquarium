use crate::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct World {
    pub bodies: Vec<RigidBody>,
}

impl World {
    pub(crate) fn test() -> Self {
        let bob = RigidBody {
            mass: 1.0, //_
            position: vec2(10.0, 10.0),
            velocity: vec2::ZERO,
            acceleration: vec2::ZERO,
            rot_inertia: 1.0,
            rotation: 0.0,
            rot_velocity: 0.0,
            rot_accel: 0.0,
        };

        Self { bodies: vec![bob] }
    }

    pub(crate) fn draw(&self, out: &mut Out) {
        draw_background(out);

        self.bodies.iter().for_each(|b| draw_body(out, b));
    }

    pub(crate) fn tick(&mut self) {
        //self.critters.iter_mut().for_each(Critter::tick);
        let dt = 0.01;
        let force = vec2f(0.0, 1.0);
        let torque = 0.0;
        let can_walk = |pos: vec2f| pos.y() < 200.0;
        for _i in 0..10 {
            self.bodies.iter_mut().for_each(|b| b.tick(dt, force, torque, can_walk));
        }
    }
}

fn draw_body(out: &mut Out, body: &RigidBody) {
    let pos = body.position.as_i32();
    let s = vec2(2, 2);
    out.draw_rect_screen(L_SPRITES, Rectangle::new((pos - s, pos + s), RGBA::WHITE));
}

fn draw_background(out: &mut Out) {
    let (w, h) = out.viewport_size.as_i32().into();
    let bg = [0, 0, 80];
    out.draw_rect_screen(0, Rectangle::from((((0, 0), (w, h)), bg)).with_fill(bg));
}
