use crate::prelude::*;

#[derive(Serialize, Deserialize, Default)]
pub struct DebugOpts {
    pub show_walkable: bool,
    pub show_buildable: bool,
    pub show_destination: bool,
    pub show_home: bool,
    pub draw_mouse: bool,
    pub pause_on_sanity_failure: bool,
    pub inspect_under_cursor: bool,
}

pub(super) fn draw_debug_overlay(g: &G, out: &mut Out) {
    if g.debug.show_walkable {
        draw_tile_overlay(g, out, RGBA([255, 0, 0, 100]), |p| !g.is_walkable(p));
    }
    if g.debug.show_buildable {
        draw_tile_overlay(g, out, RGBA([255, 0, 0, 100]), |p| !g.is_buildable(p));
    }
    if g.debug.show_home {
        draw_home_overlay(g, out);
    }
    if g.debug.show_destination {
        draw_destinations(g, out);
    }
    if g.debug.draw_mouse {
        out.draw_sprite(g, L_UI_FG + 1, sprite!("pointer"), g.mouse_position_world());
    }
    if g.debug.inspect_under_cursor {
        inspect_under_cursor(g, out);
    }
}

fn inspect_under_cursor(g: &G, out: &mut Out) {
    if let Some(pawn) = g.pawn_at(g.mouse_tile()) {
        writeln!(&mut out.debug, "{pawn:?}").ignore_err();
    }

    if let Some(building) = g.building_at(g.mouse_tile()) {
        writeln!(&mut out.debug, "{building:?}").ignore_err();
    }

    let level = g.water_level_at(g.mouse_tile());
    if level != 0.0 {
        writeln!(&mut out.debug, "water level: {level}").ignore_err();
    }
}

/// ❎ Draw a patch over all tiles where `f()` is `true`.
/// E.g. to debug all tiles that are walkable, buildable, etc.
fn draw_tile_overlay(g: &G, out: &mut Out, color: RGBA, f: impl Fn(vec2i16) -> bool) {
    for (idx, _) in visible_tiles(g) {
        if f(idx) {
            let bounds = Bounds2D::from_pos_size(idx.pos(), TILE_VSIZE).translated(-g.camera_pos);
            out.push_rect_screen(L_SPRITES + 1, Rectangle::new(bounds, color).with_fill(color));
        }
    }
}

fn draw_destinations(g: &G, out: &mut Out) {
    for pawn in g.pawns() {
        if let Some(destination) = pawn.route.destination()
            && !pawn.is_at_destination()
        {
            out.push_line(L_SPRITES, Line::new(pawn.center(), destination.pos() + TILE_ISIZE / 2).with_color(RGB::WHITE.with_alpha(128)).translated(-g.camera_pos));
        }
    }
}

fn draw_home_overlay(g: &G, out: &mut Out) {
    let color = RGBA::new(0, 0, 255, 100);
    for pawn in visible_pawns(g) {
        if let Some(home) = g.buildings.get_maybe(pawn.home.get()) {
            out.push_line(L_SPRITES + 1, Line::new(pawn.center(), home.tile.pos()).with_color(color).with_width(2).translated(-g.camera_pos));
        }
    }
}
