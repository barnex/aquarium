use crate::prelude::*;
use enum_dispatch::enum_dispatch;

/// Short-lived visual effects.
#[derive(Default)]
pub struct Effects {
    effects: RefCell<Vec<(u64, Effect)>>,
}

#[enum_dispatch]
pub enum Effect {
    Bolt,
    Crater,
}

#[enum_dispatch(Effect)]
trait EffectT {
    fn draw(&self, out: &mut Out);
    fn ttl(&self) -> u64;
}

pub struct Bolt {
    start: vec2i,
    end: vec2i,
}

impl EffectT for Bolt {
    fn draw(&self, out: &mut Out) {
        out.draw_line(L_EFFECTS, Line::new(self.start, self.end).with_color(RGBA::YELLOW.with_alpha(128)).with_width(3));
    }
    fn ttl(&self) -> u64 {
        1
    }
}

pub struct Crater {
    tile: vec2i16,
}

impl EffectT for Crater {
    fn draw(&self, out: &mut Out) {
        out.draw_sprite(L_EFFECTS, sprite!("crater"), self.tile.pos());
    }
    fn ttl(&self) -> u64 {
        30
    }
}

impl Effects {
    pub fn add_bolt(&self, g: &G, start: vec2i, end: vec2i) {
        self.add(g, Bolt { start, end })
    }

    pub fn add(&self, g: &G, effect: impl Into<Effect>) {
        let effect = effect.into();
        self.effects.borrow_mut().push((g.curr_sim_tick + effect.ttl(), effect));
    }

    pub fn tick_and_draw(&self, g: &G, out: &mut Out) {
        let mut effects = self.effects.borrow_mut();

        effects.retain(|(eol, _)| *eol > g.curr_sim_tick);

        for (_, b) in effects.iter() {
            b.draw(out)
        }
    }

    pub fn add_crater(&self, g: &G, tile: vec2i16) {
        self.add(g, Crater { tile });
    }
}
