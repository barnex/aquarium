use crate::prelude::*;

/// Short-lived visual effects.
#[derive(Default)]
pub struct Effects {
    // TODO: TTL
    lines: RefCell<Vec<Line>>,
}

impl Effects {
    pub fn add_line(&self, line: Line) {
        self.lines.borrow_mut().push(line);
    }

    pub fn draw(&self, g: &G, out: &mut Out) {
        for l in self.lines.borrow().iter() {
            out.draw_line_screen(L_SPRITES, l.clone().translated(-g.camera_pos));
        }
    }
}
