//! User Control (select, give orders, build, ...).
//!
//! Independent of time/ticks. Commands can always be given
//! (but will only carry out when time progresses).
use crate::prelude::*;

/// 🖱️ Contextual action, happens when right-clicking.
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
    /// 🖱️ User inputs give commands to the world.
    pub fn command_game(&mut self) {
        control_camera(self);
        doodle_on_map(self);

        drag_to_select(self);
        update_contextual_action(self);
        command_selected_entities(self);
    }
}

/// 🖱️ 𛲝   Drag mouse to select entities.
fn drag_to_select(g: &mut G) {
    if g.ui.active_tool != Tool::Pointer {
        return;
    }

    if g.inputs.just_pressed(K_MOUSE1) {
        g.selection_start = Some(g.mouse_position_world())
    }

    if g.inputs.just_released(K_MOUSE1) {
        if let Some(start) = g.selection_start {
            g.selected_entity_ids.clear();

            let end = g.mouse_position_world();
            let selection = Bounds2D::new_sorted(start.to_tile(), end.to_tile());
            let selection = selection.with(|s| s.max += 1);

            for e in g.entities() {
                if selection.overlaps(&e.bounds()) {
                    g.select_entity(e.id()) //
                }
            }
            log::trace!("selected {} entitites", g.selected_entity_ids.len())
        }
        g.selection_start = None;
    }
}

/// 🎯 Update g.contextual_action (what to do on right click: move, assign, attack,...).
/// Action depends on selected units and thing under cursor.
fn update_contextual_action(g: &mut G) {
    if g.ui.active_tool != Tool::Pointer {
        g.contextual_action = Action::None;
    }

    let mouse = g.mouse_tile();

    // ⏬ Assign pawns to factory.
    // Only show Assign action if some pawns can be assigned.
    //log::warn!("todo");
    //if let Some(building) = g.entities_at::<Building>(mouse).next() {
    if g.entities_at::<Building>(mouse).next().is_some() {
        // TODO: check if can assign
        //if g.selected_entities().any(|p| p.can_assign_to(&building)) {
        return g.contextual_action = Action::Assign;
        //}
    }

    // 🥾 Move to location
    // Only show Move action if some pawns can move.
    if g.selected_entities().any(|e| e.can_move()) {
        return g.contextual_action = Action::Move;
    }

    g.contextual_action = Action::None;
}

//🖱️
fn command_selected_entities(g: &mut G) {
    if g.ui.active_tool == Tool::Pointer {
        if g.inputs.just_pressed(K_MOUSE2) {
            log::trace!("contextual_action: {:?}", g.contextual_action);
            let mouse = g.mouse_tile();
            match g.contextual_action {
                Action::None => (),
                Action::Move => g.selected_entities().filter_map(|e| e.downcast::<Pawn>()).for_each(|e| e.set_destination(g, mouse).ignore()),
                Action::Assign => {
                    log::warn!("TODO");
                    //if let Some(building) = g.building_at(mouse) {
                    //    g.selected_pawns().for_each(|pawn| g.assign_to(pawn, building));
                    //}
                }
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
            Tool::Pawn2(typ, team) => {
                if g.inputs.just_pressed(K_MOUSE1) {
                    log::trace!("player spawns pawn {typ:?} {team:?} @ {mouse}");
                    if g.dyn_entities_at(mouse).next().is_none() {
                        g.spawn(Pawn::new(typ, mouse, team));
                    } else {
                        log::trace!("cannot spawn at @ {mouse}: already occupied");
                    }
                }
            }
            Tool::Building(typ) => {
                if g.inputs.just_pressed(K_MOUSE1) {
                    log::trace!("player spawns building {typ:?} @ {mouse}");
                    let team = match typ {
                        BuildingTyp::StarNest => Team::Pests,
                        _ => g.player,
                    };
                    g.spawn(Building::new(typ, mouse, team));
                }
            }
            Tool::Resource(typ) => {
                if g.inputs.just_pressed(K_MOUSE1) {
                    log::trace!("player spawns resource {typ:?} @ {mouse}");
                    g.spawn_resource(g.mouse_tile(), typ);
                }
            }
            Tool::WaterBucket => {
                if g.inputs.is_down(K_MOUSE1) && g.tile_at(mouse) == Tile::Canal {
                    log::trace!("player insert water @ {mouse}");
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
