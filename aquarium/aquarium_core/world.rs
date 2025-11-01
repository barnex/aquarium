use crate::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct World {
    pub bones: Vec<Bone>,
    //pub springs: Vec<Spring>,
}

impl World {
    pub(crate) fn test() -> Self {
        let mass = 1.0;
        let rot_inertia = 4.0;
        let len = 60.0;
        let leg1 = Bone::new(mass, rot_inertia, len).with(|v| v.body.position = vec2f(70.0, 50.0));
        let bones = (0..20).map(|i| Bone::new(mass, rot_inertia, len).with(|v| v.body.position = vec2f(-(i as f32) * len, 50.0))).collect();

        Self { bones }
    }

    pub(crate) fn draw(&self, out: &mut Out) {
        draw_background(out);

        for b in &self.bones {
            draw_bone(out, b)
        }
    }

    pub(crate) fn tick(&mut self) {
        for _i in 0..50 {
            self.minor_tick();
        }
    }

    pub(crate) fn minor_tick(&mut self) {
        //self.critters.iter_mut().for_each(Critter::tick);
        let dt = 0.02;
        let can_walk = |pos: vec2f| pos.y() < 200.0;

        for i in 0..self.bones.len() {


            self.bones[i].body.force = vec2f(0.0, 0.0);
            self.bones[i].body.torque = 0.0;

            let b = &self.bones[i];

            if i > 0 {
                let anchor1 = vec2(-b.len / 2.0, 0.0);
                let anchor2 = vec2(b.len / 2.0, 0.0);

                let neigh = self.bones[i - 1].body.transform_rel_pos(anchor1);

                let k = 0.02;
                let spring_force = k * (neigh - b.body.transform_rel_pos(anchor2));

                self.bones[i].body.torque += -0.03 * cross(b.body.transform_vector(anchor2), spring_force); // LEFT HANDED !!
                //log::trace!("torque: {torque}");

                self.bones[i].body.force += spring_force;
            }

            let b = &mut self.bones[i];

            b.body.tick(dt);

            b.body.velocity *= 0.99;
            b.body.rot_velocity *= 0.99;
        }
    }
}

fn cross(a: vec2f, b: vec2f) -> f32 {
    a.x() * b.y() - a.y() * b.x()
}

fn draw_bone(out: &mut Out, bone: &Bone) {
    draw_body(out, &bone.body);
    let color = RGBA::YELLOW;
    let start = bone.body.transform_rel_pos(vec2(-bone.len / 2.0, 0.0)).as_i32();
    let end = bone.body.transform_rel_pos(vec2(bone.len / 2.0, 0.0)).as_i32();
    out.draw_line_screen(L_SPRITES, Line::new(start, end).with_color(color));
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
