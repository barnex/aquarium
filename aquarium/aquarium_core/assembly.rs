use crate::prelude::*;

/// A contraption made of rigid bodies connected via springs.
#[derive(Serialize, Deserialize)]
pub struct Assembly {
    pub g: f32,
    pub bones: Vec<RigidBody>,
    pub bone_len: f32,
    pub springs: Vec<Spring>,
}

impl Assembly {
    pub(crate) fn test(n: usize) -> Self {
        let mass = 1.0;
        let rot_inertia = 300.0;
        let bone_len = 5.0;
        //let leg1 = Bone::new(mass, rot_inertia, len).with(|v| v.body.position = vec2f(70.0, 50.0));

        let mut bones = (0..n).map(|i| RigidBody::new(mass, rot_inertia).with(|v| v.position = vec2f(600.0 - (i as f32) * bone_len, 150.0))).collect_vec();

        bones[0].mass = 10.0;

        let mut springs = (0..(bones.len()))
            //.circular_tuple_windows()
            .tuple_windows()
            .map(|(ia, ib)| Spring {
                ia,
                ib,
                anchor_a: vec2(-bone_len / 2.0, 0.0),
                anchor_b: vec2(bone_len / 2.0, 0.0),
                k: 10.0,
            })
            .collect_vec();

        //springs[0].anchor_a = vec2(0.0, 0.0);

        Self { bone_len, bones, springs, g: 0.01 }
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
        for _i in 0..40 {
            self.minor_tick();
        }
    }

    pub(crate) fn minor_tick(&mut self) {
        self.update_forces();
        self.verlet_tick();
    }

    fn verlet_tick(&mut self) {
        let dt = 0.01;

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
            let ia = spring.ia;
            let ib = spring.ib;

            let anchor_a = self.bones[ia].transform_rel_pos(spring.anchor_a);
            let anchor_b = self.bones[ib].transform_rel_pos(spring.anchor_b);

            let force_a = spring.k * (anchor_b - anchor_a);
            let force_b = -force_a;

            let torque_a = -cross(self.bones[ia].transform_vector(spring.anchor_a), force_a); // LEFT HANDED !!
            let torque_b = -cross(self.bones[ib].transform_vector(spring.anchor_b), force_b); // LEFT HANDED !!

            let dir_a = self.bones[ia].transform_vector(vec2::EX);
            let dir_b = self.bones[ib].transform_vector(vec2::EX);
            let restore = 10.0 * cross(dir_b, dir_a).powi(3);

            let torque_a = torque_a + restore;
            let torque_b = torque_b - restore;

            self.bones[ia].torque += torque_a;
            self.bones[ib].torque += torque_b;

            self.bones[ia].force += force_a;
            self.bones[ib].force += force_b;

            // laplacian
        }
    }

    fn draw_spring(&self, out: &mut Out, i: usize) {
        let spring = &self.springs[i];
        let color = RGBA::YELLOW;

        let ia = spring.ia;
        let ib = spring.ib;

        let anchor_a = self.bones[ia].transform_rel_pos(spring.anchor_a);
        let anchor_b = self.bones[ib].transform_rel_pos(spring.anchor_b);

        out.draw_line_screen(L_SPRITES, Line::new(anchor_a.as_(), anchor_b.as_()).with_color(color).with_width(3));
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
