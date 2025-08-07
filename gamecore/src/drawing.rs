//! Draw the world.
use crate::prelude::*;

impl G {
    pub fn draw_world(&self, out: &mut Out) {
        debug_assert!(self.viewport_size != vec2::ZERO);
        // Note: ⚠️ UI already rendered (may consume input events)

        let g = self;
        draw_tilemap(g, out);
        draw_buildings(g, out);
        draw_pawns(g, out);
        draw_cursor(g, out);
        draw_selection(g, out);
        draw_destinations(g, out);

        draw_debug_overlay(g, out);
    }
}

fn visible_tiles(g: &G) -> impl Iterator<Item = (vec2i16, Tile)> {
    // 🪲 TODO: restrict to viewport
    g.tilemap.enumerate_all()
}

fn visible_pawns(g: &G) -> impl Iterator<Item = &Pawn> {
    // 🪲 TODO: restrict to viewport
    g.pawns.iter()
}

fn draw_tilemap(g: &G, out: &mut Out) {
    for (idx, mat) in visible_tiles(g) {
        out.push_sprite(L_TILES, mat.sprite(), idx.pos() - g.camera_pos);
    }
}

fn draw_debug_overlay(g: &G, out: &mut Out) {
    if g.debug.show_walkable {
        draw_tile_overlay(g, out, RGBA([255, 0, 0, 100]), |p| !g.is_walkable(p));
    }
    if g.debug.show_buildable {
        draw_tile_overlay(g, out, RGBA([255, 0, 0, 100]), |p| !g.is_buildable(p));
    }
    if g.debug.show_home {
        draw_home_overlay(g, out);
    }
}

/// ❎ Draw a patch over all tiles where `f()` is `true`.
/// For debug. E.g. show all tiles that are walkable, buildable, etc.
fn draw_tile_overlay(g: &G, out: &mut Out, color: RGBA, f: impl Fn(vec2i16) -> bool) {
    for (idx, _) in visible_tiles(g) {
        if f(idx) {
            let bounds = Bounds2D::from_pos_size(idx.pos(), TILE_VSIZE).translated(-g.camera_pos);
            out.push_rect(L_SPRITES + 1, Rectangle::new(bounds, color).with_fill(color));
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

fn draw_buildings(g: &G, out: &mut Out) {
    for building in g.buildings.iter() {
        out.push_sprite(L_SPRITES, building.typ.sprite(), building.tile.pos() - g.camera_pos);
    }
}

fn draw_pawns(g: &G, out: &mut Out) {
    for pawn in g.pawns.iter() {
        out.push_sprite(L_SPRITES, pawn.typ.sprite(), pawn.tile.pos() - g.camera_pos);
    }
}

fn draw_cursor(g: &G, out: &mut Out) {
    let sprite = match g.ui.active_tool {
        Tool::Pointer => sprite!("grid24"),
        Tool::Tile(typ) => typ.sprite(),
        Tool::Pawn(typ) => typ.sprite(),
        Tool::Building(typ) => typ.sprite(),
    };
    out.push_sprite(L_SPRITES, sprite, g.mouse_tile().pos() - g.camera_pos);
    out.push_sprite(L_SPRITES, sprite!("grid24"), g.mouse_tile().pos() - g.camera_pos);
}

fn draw_selection(g: &G, out: &mut Out) -> Option<()> {
    if let Some(start) = g.selection_start {
        let end = g.mouse_position_world();

        let min = start.zip_with(end, i32::min);
        let max = start.zip_with(end, i32::max);
        let sel = Bounds2D::new(min, max);

        out.push_rect(L_SPRITES, Rectangle::new(sel.translated(-g.camera_pos), RGBA::BLUE).with_fill(RGB::BLUE.with_alpha(64)));
    }

    for pawn in g.selected_pawns() {
        out.push_rect(L_SPRITES, Rectangle::new(pawn.bounds().translated(-g.camera_pos), RGBA::BLUE).with_fill(RGB::BLUE.with_alpha(64)));
    }
    OK
}

fn draw_destinations(g: &G, out: &mut Out) {
    for pawn in g.selected_pawns() {
        if let Some(destination) = pawn.route.destination()
            && !pawn.is_at_destination()
        {
            out.push_line(L_SPRITES, Line::new(pawn.center(), destination.pos() + TILE_ISIZE / 2).with_color(RGB::WHITE.with_alpha(128)).translated(-g.camera_pos));
        }
    }
}
