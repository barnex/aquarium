use crate::prelude::*;

/// A contraption made of rigid bodies connected via springs.
#[derive(Serialize, Deserialize)]
pub struct Contraption {
    pub g: f32,
    pub bones: Vec<RigidBody>,
    pub bone_len: f32,
    pub stiffness: f32,
    pub springs: Vec<Spring>,
}

impl Contraption {
    pub fn rope(n: usize) -> Self {
        let mass = 1.0;
        let bone_len = 15.0f32;

        let rot_inertia = mass * bone_len.powi(2); // approx. Ideally should be chosen so that rotational and translational frequencies are equal (for most efficient time step).

        let bones = (0..n).map(|i| RigidBody::new(mass, rot_inertia).with(|v| v.position = vec2f(600.0 - (i as f32) * bone_len, 150.0))).collect_vec();

        let springs = (0..(bones.len()))
            .tuple_windows()
            .map(|(ia, ib)| Spring {
                ia,
                ib,
                anchor_a: vec2(-bone_len / 2.0, 0.0),
                anchor_b: vec2(bone_len / 2.0, 0.0),
                k: 10.0,
                sin_angle: 0.0,
            })
            .collect_vec();

        Self {
            bone_len,
            bones,
            springs,
            g: 0.0,
            stiffness: 50.0,
        }
    }

    pub fn draw(&self, out: &mut Out) {
        for b in &self.bones {
            self.draw_bone(out, b)
        }

        for s in 0..self.springs.len() {
            self.draw_spring(out, s)
        }
    }

    pub(crate) fn tick(&mut self) {
        for _i in 0..10 {
            self.minor_tick();
        }
    }

    pub(crate) fn minor_tick(&mut self) {
        self.update_forces();
        self.damping_tick();
    }

    fn damping_tick(&mut self) {
        let dt = 0.03;
        self.bones.iter_mut().for_each(|b| b.update_accel()); //          |
        self.bones.iter_mut().for_each(|b| b.dampen_position(dt)); //     |
    }

    fn verlet_tick(&mut self) {
        let dt = 0.03;

        //                                                      <-----<--------
        //self.bones[0].body.position = vec2(600.0, 150.0); //                   ^
        self.bones.iter_mut().for_each(|b| b.update_accel()); //          |
        self.bones.iter_mut().for_each(|b| b.update_velocity(dt)); //     |
        self.dampen(); //                                                      |
        //                                                                     ^
        // logically, the cycle starts here:                                   |
        self.bones.iter_mut().for_each(|b| b.update_velocity_half(dt)); //|
        self.bones.iter_mut().for_each(|b| b.update_position(dt)); //     |
        //                                                       >------>------^
    }

    fn dampen(&mut self) {
        for b in &mut self.bones {
            b.velocity *= 0.9995;
            b.rot_velocity *= 0.9995;
        }
    }

    fn update_forces(&mut self) {
        let g = self.g;
        for bone in &mut self.bones {
            bone.force = vec2(0.0, g);
            //bone.body.force = default();
            bone.torque = default();
        }

        for spring in &self.springs {
            // the two bones connected by the spring
            let Ok([bone_a, bone_b]) = self.bones.get_disjoint_mut([spring.ia, spring.ib]) else { panic!("self-connected spring") };

            // positions of spring ends
            let anchor_a = bone_a.transform_rel_pos(spring.anchor_a);
            let anchor_b = bone_b.transform_rel_pos(spring.anchor_b);

            // spring forces on both bones (via spring constant k)
            let force_a = spring.k * (anchor_b - anchor_a);
            let force_b = -force_a;

            // torques exerted by spring forces
            let torque_a = -cross(bone_a.transform_vector(spring.anchor_a), force_a); // LEFT HANDED !!
            let torque_b = -cross(bone_b.transform_vector(spring.anchor_b), force_b); // LEFT HANDED !!

            // additional torque that tries to straighten out the connection (via stiffness).
            let dir_a = bone_a.transform_vector(vec2::EX);
            let dir_b = bone_b.transform_vector(vec2::EX);
            let stiffness_torque = self.stiffness * (cross(dir_b, dir_a) + spring.sin_angle);
            let torque_a = torque_a + stiffness_torque;
            let torque_b = torque_b - stiffness_torque;

            // finally, add this spring's forces and torques to the bones
            bone_a.torque += torque_a;
            bone_b.torque += torque_b;

            bone_a.force += force_a;
            bone_b.force += force_b;
        }
    }

    fn draw_spring(&self, out: &mut Out, i: usize) {
        let spring = &self.springs[i];
        let color = RGBA::RED;

        let ia = spring.ia;
        let ib = spring.ib;

        let anchor_a = self.bones[ia].transform_rel_pos(spring.anchor_a);
        let anchor_b = self.bones[ib].transform_rel_pos(spring.anchor_b);

        out.draw_line_screen(L_SPRITES, Line::new(anchor_a.as_(), anchor_b.as_()).with_color(color).with_width(2));
    }

    fn draw_bone(&self, out: &mut Out, bone: &RigidBody) {
        let bone_len = self.bone_len;
        let color = RGBA::YELLOW;
        let start = bone.transform_rel_pos(vec2(-bone_len / 2.0, 0.0)).as_i32();
        let end = bone.transform_rel_pos(vec2(bone_len / 2.0, 0.0)).as_i32();
        out.draw_line_screen(L_SPRITES, Line::new(start, end).with_color(color).with_width(3));
    }
}

fn cross(a: vec2f, b: vec2f) -> f32 {
    a.x() * b.y() - a.y() * b.x()
}
