use crate::prelude::*;

#[derive(Serialize, Deserialize, Default)]
pub struct DebugOpts {
    pub show_walkable: bool,
    pub show_buildable: bool,
    pub show_destination: bool,
    pub show_home: bool,
    pub show_downstream: bool,
    pub draw_mouse: bool,
    pub pause_on_sanity_failure: bool,
    pub inspect_under_cursor: bool,
}

pub(super) fn draw_debug_overlay(g: &G, out: &mut Out) {
    if g.debug.show_walkable {
        draw_tile_overlay(g, out, RGBA([255, 0, 0, 100]), |p| !g.tile_at(p).is_default_walkable());
    }
    if g.debug.show_buildable {
        // TODO
        //draw_tile_overlay(g, out, RGBA([255, 0, 0, 100]), |p| !g.is_buildable(p, BuildingTyp::HQ));
    }
    if g.debug.show_home {
        draw_home_overlay(g, out);
    }
    if g.debug.show_downstream {
        draw_downstream_overlay(g, out);
    }
    if g.debug.show_destination {
        draw_destinations(g, out);
    }
    if g.debug.draw_mouse {
        out.draw_sprite(L_UI_FG + 1, sprite!("pointer"), g.mouse_position_world());
    }
    if g.debug.inspect_under_cursor {
        inspect_under_cursor(g, out);
    }
}

fn inspect_under_cursor(g: &G, out: &mut Out) {
    let mouse = g.mouse_tile();
    if let Some(pawn) = g.pawn_at(mouse) {
        writeln!(&mut out.debug, "{pawn:?}").ignore_err();
    }

    if let Some(building) = g.building_at(mouse) {
        writeln!(&mut out.debug, "{building:?}").ignore_err();
    }

    writeln!(&mut out.debug, "water level: {}", g.water_level_at(mouse)).ignore_err();
    //writeln!(&mut out.debug, "water v left: {:?}", g.water.velocity_left_of.get(&mouse)).ignore_err();
}

/// ❎ Draw a patch over all tiles where `f()` is `true`.
/// E.g. to debug all tiles that are walkable, buildable, etc.
fn draw_tile_overlay(g: &G, out: &mut Out, color: RGBA, f: impl Fn(vec2i16) -> bool) {
    for (idx, _) in visible_tiles(g) {
        if f(idx) {
            let bounds = Bounds2D::from_pos_size(idx.pos(), TILE_VSIZE).translated(-g.camera_pos);
            out.draw_rect_screen(L_SPRITES + 1, Rectangle::new(bounds, color).with_fill(color));
        }
    }
}

fn draw_destinations(g: &G, out: &mut Out) {
    for pawn in g.pawns() {
        if let Some(destination) = pawn.route.destination()
            && !pawn.is_at_destination()
        {
            out.draw_line(L_SPRITES, Line::new(pawn.center(), destination.pos() + TILE_ISIZE / 2).with_color(RGB::WHITE.with_alpha(128)));
        }
    }
}

fn draw_home_overlay(g: &G, out: &mut Out) {
    let color = RGBA::new(0, 0, 255, 100);
    for pawn in visible_pawns(g) {
        if let Some(home) = pawn.home.get().and_then(|id| g.get::<Building>(id)) {
            out.draw_line(L_SPRITES + 2, Line::new(pawn.center(), home.tile().pos()).with_color(color).with_width(2));
        }
    }
}

fn draw_downstream_overlay(g: &G, out: &mut Out) {
    //for src in visible_buildings(g) {
    //    for dst in src.downstream_buildings(g) {
    //        out.draw_line(L_SPRITES + 1, Line::new(src.tile_bounds().max.pos(), dst.entrance().pos()));
    //    }
    //}
}

pub(crate) fn write_debug_output(g: &G, out: &mut Out) {
    let debug = &mut out.debug;
    if let Some(e) = g.last_sanity_error.as_ref() {
        writeln!(debug, "SANITY CHECK FAILED: {e}").ignore_err();
    }

    writeln!(debug, "now: {:.04}s, frame: {}, tick: {}, FPS: {:.01}", g.now_secs, g.frame, g.tick, 1.0 / g.dt_smooth).unwrap();

    let total_water = g.water.h.values().sum::<f32>();
    let max_water = g.water.h.values().copied().max_by(|a, b| a.total_cmp(b)).unwrap_or_default();
    let total_momentum = g.water.p.values().copied().sum::<vec2f>();
    let max_momentum = g.water.p.values().copied().max_by(|a, b| a.len2().total_cmp(&b.len2())).unwrap_or_default();

    writeln!(debug, "total water: {total_water:.6} (max {max_water:.6}), momentum: {:.6},{:.6} (max {max_momentum})", total_momentum.x(), total_momentum.y()).unwrap();

    writeln!(debug, "camera {:?}", g.camera_pos).unwrap();
    writeln!(debug, "down {:?}", g.inputs.iter_is_down().sorted().collect_vec()).unwrap();
    writeln!(debug, "tile_picker {:?}", g.ui.active_tool).unwrap();
    writeln!(debug, "selected: {:?}", g.selected_entity_ids.len()).unwrap();
    writeln!(debug, "contextual_action: {:?}", g.contextual_action).unwrap();
    writeln!(debug, "draw commands: {}", out.layers.iter().map(|l| l.lines.len() + l.rectangles.len() + l.sprites.len()).sum::<usize>()).unwrap();
    writeln!(debug, "map size: {} ({} tiles)", g._tilemap.size(), g._tilemap.size().as_u32().product()).unwrap();
}
