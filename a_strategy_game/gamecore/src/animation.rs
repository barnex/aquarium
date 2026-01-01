use std::mem;

use crate::prelude::*;

/// Computes smooth animations in between simulation ticks.
///
/// Simulation ticks are very coarse (e.g. 200ms), but we want
/// to animate Entity positions on the screen smoothly. E.g. a moving crab should
/// not "jump" from one tile to the next every 200ms, but smoothly slide between them.
///
/// Animation is only needed for currently visible Entities (in the camera's viewport).
/// Therefore, we store the needed state (previous position etc.) not in the Entity
/// itself, but here in the `AnimationState`.
///
/// Drawback: while scrolling, newly visible Entities will only start animating after one tick
/// (because their previous position was not yet tracked). This is not noticable.
#[derive(Default)]
pub struct AnimationState {
    /// Current simulation tick, as passed via `next_frame`.
    curr_tick: u64,
    /// When the current simulation tick started.
    curr_tick_start_micros: u64,

    /// Fractional part of the tick, linearly goes from 0.0 to 1.0 over the course of each tick.
    /// Interpolation parameter for smooth animations in between ticks.
    fractional_tick: f64,

    /// Entity positions at the previous tick, keyed by Entity Id.
    prev_pos: HashMap<Id, vec2i>,

    /// Entity positions at the current tick, keyed by Entity Id.
    /// Upon `next_frame`, these become `prev_pos`.
    curr_pos: HashMap<Id, vec2i>,
}

impl AnimationState {
    /// Must be called each time before rendering a frame.
    /// Updates the internal state needed to animate positions when drawing (see `smooth_pos()`).
    pub fn next_frame(&mut self, micros_per_tick: u64, curr_tick: u64, curr_micros: u64) {
        if curr_tick != self.curr_tick {
            // New simulation tick. Interpolation resets to 0.0.
            self.curr_tick = curr_tick;
            self.curr_tick_start_micros = curr_micros;
            // Current Entity positions become previous.
            mem::swap(&mut self.prev_pos, &mut self.curr_pos);
            self.curr_pos.clear();
        }

        // Linearly advance interpolation from 0.0 to 1.0 during the simulation tick.
        self.fractional_tick = (((curr_micros - self.curr_tick_start_micros) as f64) / (micros_per_tick as f64));

        // Fail-safe in case of numerical weirdness. Should not be needed.
        if !self.fractional_tick.is_finite() {
            self.fractional_tick = 1.0;
        }
        // If a tick arrives late (e.g. high CPU load),
        // then don't interpolate out-of-range (>1.0).
        self.fractional_tick = self.fractional_tick.clamp(0.0, 1.0);
    }

    /// Compute smoothed pixel position of entity with given Id and position in the current simulation tick.
    /// This linearly interpolates from the previous position to the current position over the duration of one simulation tick.
    ///
    /// If the previous position was not yet recorded, then the current position is returned
    /// (i.e. just draw at current position without animation -- will animate smoothly from the next tick onward).
    pub fn smooth_pos(&mut self, id: Id, pos: vec2i) -> vec2i {
        self.curr_pos.insert(id, pos);
        let prev = self.prev_pos.get(&id).copied().unwrap_or(pos);
        lerp(prev.as_f64(), pos.as_f64(), self.fractional_tick).as_i32()
    }
}
