use crate::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct World {
    pub bones: Vec<Bone>,
}

impl World {
    pub(crate) fn test() -> Self {
        let mass = 1.0;
        let rot_inertia = 1.0;
        let len = 60.0;
        let leg1 = Bone::new(mass, rot_inertia, len).with(|v| v.body.position = vec2f(50.0, 50.0));
        let leg2 = RigidBody::new(mass, rot_inertia).with(|v| v.position = vec2f(20.0, 0.0)).with(|v| v.velocity = vec2f(3.0, 0.0));

        Self { bones: vec![leg1] }
    }

    pub(crate) fn draw(&self, out: &mut Out) {
        draw_background(out);

        for b in &self.bones {
            draw_bone(out, b)
        }
    }

    pub(crate) fn tick(&mut self) {
        //self.critters.iter_mut().for_each(Critter::tick);
        let dt = 0.01;
        let force = vec2f(0.0, 1.0);
        let torque = 0.0;
        let can_walk = |pos: vec2f| pos.y() < 200.0;

        for _i in 0..10 {
            self.bones.iter_mut().for_each(|b| b.body.tick(dt, force, torque, can_walk));
        }
    }
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
