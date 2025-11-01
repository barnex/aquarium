use crate::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct World {
    pub g: f32,
    pub bones: Vec<Bone>,
    pub springs: Vec<Spring>,
}

#[derive(Serialize, Deserialize)]
pub struct Spring {
    pub ia: usize,
    pub ib: usize,
    pub anchor_a: vec2f,
    pub anchor_b: vec2f,
    pub k: f32,
}

impl World {
    pub(crate) fn test(n: usize) -> Self {
        let mass = 1.0;
        let rot_inertia = 300.0;
        let len = 10.0;
        //let leg1 = Bone::new(mass, rot_inertia, len).with(|v| v.body.position = vec2f(70.0, 50.0));

        let mut bones = (0..n).map(|i| Bone::new(mass, rot_inertia, len).with(|v| v.body.position = vec2f(600.0 - (i as f32) * len, 150.0))).collect_vec();

        //bones[0].len = 0.001;
        bones[0].body.mass = 10000.0;

        let mut springs = (0..(bones.len()))
            //.circular_tuple_windows()
            .tuple_windows()
            .map(|(ia, ib)| Spring {
                ia,
                ib,
                anchor_a: vec2(-len / 2.0, 0.0),
                anchor_b: vec2(len / 2.0, 0.0),
                k: 10.0,
            })
            .collect_vec();

        //springs[0].anchor_a = vec2(0.0, 0.0);

        Self { bones, springs, g: 0.01 }
    }

    pub(crate) fn draw(&self, out: &mut Out) {
        draw_background(out);

        for b in &self.bones {
            draw_bone(out, b)
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
        self.bones.iter_mut().for_each(|b| b.body.update_accel()); //          |
        self.bones.iter_mut().for_each(|b| b.body.update_velocity(dt)); //     |
        self.dampen(); //                                                      |
        //                                                                     ^
        // logically, the cycle starts here:                                   |
        self.bones.iter_mut().for_each(|b| b.body.update_velocity_half(dt)); //|
        self.bones.iter_mut().for_each(|b| b.body.update_position(dt)); //     |
        //                                                       >------>------^
    }

    fn dampen(&mut self) {
        for b in &mut self.bones {
            b.body.velocity *= 0.9995;
            b.body.rot_velocity *= 0.9995;
        }
    }

    fn update_forces(&mut self) {
        let g = self.g;
        for bone in &mut self.bones {
            bone.body.force = vec2(0.0, g);
            //bone.body.force = default();
            bone.body.torque = default();
        }

        for spring in &self.springs {
            let ia = spring.ia;
            let ib = spring.ib;

            let anchor_a = self.bones[ia].body.transform_rel_pos(spring.anchor_a);
            let anchor_b = self.bones[ib].body.transform_rel_pos(spring.anchor_b);

            let force_a = spring.k * (anchor_b - anchor_a);
            let force_b = -force_a;

            let torque_a = -cross(self.bones[ia].body.transform_vector(spring.anchor_a), force_a); // LEFT HANDED !!
            let torque_b = -cross(self.bones[ib].body.transform_vector(spring.anchor_b), force_b); // LEFT HANDED !!

            let dir_a = self.bones[ia].body.transform_vector(vec2::EX);
            let dir_b = self.bones[ib].body.transform_vector(vec2::EX);
            let restore = 10.0*cross(dir_b, dir_a).powi(3);

            let torque_a = torque_a + restore;
            let torque_b = torque_b - restore;

            self.bones[ia].body.torque += torque_a;
            self.bones[ib].body.torque += torque_b;

            self.bones[ia].body.force += force_a;
            self.bones[ib].body.force += force_b;

            // laplacian
        }
    }

    fn draw_spring(&self, out: &mut Out, i: usize) {
        let spring = &self.springs[i];
        let color = RGBA::YELLOW;

        let ia = spring.ia;
        let ib = spring.ib;

        let anchor_a = self.bones[ia].body.transform_rel_pos(spring.anchor_a);
        let anchor_b = self.bones[ib].body.transform_rel_pos(spring.anchor_b);

        out.draw_line_screen(L_SPRITES, Line::new(anchor_a.as_(), anchor_b.as_()).with_color(color).with_width(3));
    }
}

fn cross(a: vec2f, b: vec2f) -> f32 {
    a.x() * b.y() - a.y() * b.x()
}

fn draw_bone(out: &mut Out, bone: &Bone) {
    //draw_body(out, &bone.body);
    let color = RGBA::YELLOW;
    let start = bone.body.transform_rel_pos(vec2(-bone.len / 2.0, 0.0)).as_i32();
    let end = bone.body.transform_rel_pos(vec2(bone.len / 2.0, 0.0)).as_i32();
    out.draw_line_screen(L_SPRITES, Line::new(start, end).with_color(color).with_width(3));
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

fn draw_background(out: &mut Out) {
    let (w, h) = out.viewport_size.as_i32().into();
    let bg = [0, 0, 80];
    out.draw_rect_screen(0, Rectangle::from((((0, 0), (w, h)), bg)).with_fill(bg));
}
