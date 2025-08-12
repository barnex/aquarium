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
        draw_on_map(self);
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
                        g.building_at(mouse).map(|building| g.selected_pawns().for_each(|pawn| assign_to(g, pawn, building)));
                    }
                }
            }

            for pawn in g.selected_pawns() {
                pawn.set_destination(g, g.mouse_tile())
            }
        }
    }
}

fn assign_to(g: &G, pawn: &Pawn, building: &Building) {
    if let Some(home) = pawn.home(g) {
        home.workers.remove(&pawn.id);
    }
    building.workers.insert(pawn.id);
    pawn.home.set(Some(building.id));
}

fn draw_on_map(g: &mut G) {
    if g.inputs.is_down(K_MOUSE1) {
        if let Tool::Tile(mat) = g.ui.active_tool {}

        match g.ui.active_tool {
            Tool::Pointer => (),
            Tool::Tile(mat) => g.tilemap.set(g.mouse_tile(), mat),
            Tool::Pawn(typ) => {
                if g.inputs.just_pressed(K_MOUSE1) {
                    g.spawn(Pawn::new(typ, g.mouse_tile()));
                }
            }
            Tool::Building(typ) => {
                if g.inputs.just_pressed(K_MOUSE1) {
                    let building = Building {
                        id: default(),
                        typ,
                        tile: g.mouse_tile(),
                        workers: default(),
                    };
                    try_spawn_building(g, building);
                }
            }
            Tool::Resource(typ) => {
                if g.inputs.just_pressed(K_MOUSE1) {
                    g.resources.insert(g.mouse_tile(), typ);
                }
            }
        }
    }
}

fn try_spawn_building(g: &G, building: Building) -> Option<&Building> {
    let bounds = building.tile_bounds();
    let mut footprint = cross(bounds.x_range(), bounds.y_range());
    let can_build = footprint.all(|(x, y)| g.is_buildable(vec2(x, y)));
    if can_build { Some(g.buildings.insert(building)) } else { None }
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
