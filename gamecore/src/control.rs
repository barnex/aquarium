//! User Control (select, give orders, build, ...).
//!
//! Independent of time/ticks. Commands can always be given
//! (but will only carry out when time progresses).
use crate::prelude::*;

/// Contextual action, happens when right-clicking.
#[derive(Serialize, Deserialize, Default, PartialEq, Eq, Clone, Copy, Debug)]
#[repr(u8)]
pub enum Action {
    /// Do nothing. E.g. when no units selected.
    #[default]
    None,

    /// Move to location. E.g. when mouse above empty terrain.
    Move,

    /// Assign worker to factory.
    Assign,
}

impl G {
    /// User inputs give commands to the world.
    pub fn control(&mut self) {
        update_contextual_action(self);
        control_camera(self);
        doodle_on_map(self);
        select_pawns(self);
        command_pawns(self);
    }
}

/// 🎯 Update g.contextual_action (what to do on right click: move, assign, attack,...).
/// Action depends on selected units and thing under cursor.
fn update_contextual_action(g: &mut G) {
    if g.ui.active_tool != Tool::Pointer {
        g.contextual_action = Action::None;
    }

    let mouse = g.mouse_tile();
    let selected = !g.selected_pawn_ids.is_empty();

    // ⏬ Assign pawns to factory.
    if selected && g.building_at(mouse).is_some() {
        // TODO: and building is friendly
        return g.contextual_action = Action::Assign;
    }

    // 🥾 Move to location
    if selected {
        return g.contextual_action = Action::Move;
    }

    g.contextual_action = Action::None;
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
                    g.select_pawn(p.id) //
                }
            }
        }
        g.selection_start = None;
    }
}

fn command_pawns(g: &mut G) {
    if g.ui.active_tool == Tool::Pointer {
        if g.inputs.just_pressed(K_MOUSE2) {
            let mouse = g.mouse_tile();
            match g.contextual_action {
                Action::None => (),
                Action::Move => g.selected_pawns().for_each(|p| p.set_destination(g, mouse)),
                Action::Assign => {
                    if let Some(building) = g.building_at(mouse) {
                        g.selected_pawns().for_each(|pawn| g.assign_to(pawn, building));
                    }
                }
            }

            for pawn in g.selected_pawns() {
                pawn.set_destination(g, g.mouse_tile())
            }
        }
    }
}

fn doodle_on_map(g: &mut G) {
    if g.inputs.is_down(K_MOUSE1) {
        let mouse = g.mouse_tile();

        match g.ui.active_tool {
            Tool::Pointer => (),
            Tool::Tile(mat) => g.set_tile(mouse, mat),
            Tool::Pawn(typ) => {
                if g.inputs.just_pressed(K_MOUSE1) {
                    g.spawn(Pawn::new(typ, mouse));
                }
            }
            Tool::Building(typ) => {
                if g.inputs.just_pressed(K_MOUSE1) {
                    g.spawn_building(Building::new(typ, mouse));
                }
            }
            Tool::Resource(typ) => {
                if g.inputs.just_pressed(K_MOUSE1) {
                    g.spawn_resource(g.mouse_tile(), typ);
                }
            }
            Tool::WaterBucket => {
                if g.inputs.is_down(K_MOUSE1) && g.tile_at(mouse) == Tile::Canal {
                    g.water.h.insert(mouse, 1.0);
                    g.water.p.insert(mouse, vec2::EX); // DEBUG HACK !!!! REMOVE!!!!
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
