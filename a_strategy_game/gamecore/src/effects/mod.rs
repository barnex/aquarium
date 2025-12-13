use crate::prelude::*;

/// Short-lived visual effects.
#[derive(Default)]
pub struct Effects {
    bolts: RefCell<Vec<Bolt>>,
}

impl Effects {
    pub fn add_bolt(&self, g: &G, start: vec2i, end: vec2i) {
        self.bolts.borrow_mut().push(Bolt { born: g.tick, start, end });
    }

    pub fn tick_and_draw(&self, g: &G, out: &mut Out) {
        let mut bolts = self.bolts.borrow_mut();

        let TTL = 1; // ticks
        bolts.retain(|b| b.born + TTL > g.tick);

        for b in bolts.iter() {
            g.draw_line(out, L_EFFECTS, Line::new(b.start, b.end).with_color(RGBA::YELLOW.with_alpha(128)).with_width(3));
        }
    }
}

struct Bolt {
    born: u64,
    start: vec2i,
    end: vec2i,
}
