use crate::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct Critter {
    pub head_pos: vec2f,
}

impl Critter {
    pub(crate) fn tick(&mut self) {
        self.head_pos += 1.0;
    }

    pub(crate) fn draw(&self, out: &mut Out) {
        let pos = self.head_pos.as_i32();
        let s = vec2(2, 2);
        out.draw_rect_screen(L_SPRITES, Rectangle::new((pos - s, pos + s), RGBA::WHITE));
    }
}
