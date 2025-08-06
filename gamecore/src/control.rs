//! User Control (select, give orders, build, ...).
//!
//! Independent of time/ticks. Commands can always be given
//! (but will only carry out when time progresses).
use crate::prelude::*;

impl G {
    /// User inputs give commands to the world.
    pub fn control(&mut self) {
        control_camera(self);
        draw_on_map(self);
        select_pawns(self);
        command_pawns(self);
    }
}

fn select_pawns(g: &mut G) {
    if g.ui.active_tool != Tool::Pointer {
        return;
    }

    if g.inputs.just_pressed(K_MOUSE1) {
        g.selection_start = Some(g.mouse_position_world())
    }

    if g.inputs.just_released(K_MOUSE1) {
        if let Some(start) = g.selection_start {
            g.selected_pawn_ids.clear();

            let end = g.mouse_position_world();
            let selection = Bounds2D::new_sorted(start, end);
            let selection = selection.with(|s| s.max += 1);

            for p in g.pawns.iter() {
                if selection.overlaps(&p.bounds()) {
                    g.selected_pawn_ids.push(p.id) //
                }
            }
        }
        g.selection_start = None;
    }
}

fn command_pawns(g: &mut G) {
    if g.ui.active_tool == Tool::Pointer {
        if g.inputs.just_pressed(K_MOUSE2) {
            for pawn in g.selected_pawns() {
                pawn.set_destination(g.mouse_tile())
            }
        }
    }
}

fn draw_on_map(g: &mut G) {
    if g.inputs.is_down(K_MOUSE1) {
        if let Tool::Tile(mat) = g.ui.active_tool {}

        match g.ui.active_tool {
            Tool::Pointer => (),
            Tool::Tile(mat) => g.tilemap.set(g.mouse_tile(), mat),
            Tool::Pawn(typ) => {
                if g.inputs.just_pressed(K_MOUSE1) {
                    g.pawns.insert(Pawn::new(typ, g.mouse_tile()));
                }
            }
            Tool::Building(typ) => {
                if g.inputs.just_pressed(K_MOUSE1) {
                    g.buildings.insert(Building { id: default(), typ, tile: g.mouse_tile() });
                }
            }
        }
    }
}

fn control_camera(g: &mut G) {
    let mut delta = vec2::ZERO;
    if g.inputs.is_down(K_CAM_DOWN) {
        delta += vec2(0, 1);
    }
    if g.inputs.is_down(K_CAM_UP) {
        delta += vec2(0, -1);
    }
    if g.inputs.is_down(K_CAM_LEFT) {
        delta += vec2(-1, 0);
    }
    if g.inputs.is_down(K_CAM_RIGHT) {
        delta += vec2(1, 0);
    }
    let speed = 3;
    g.camera_pos += speed * delta;
}
