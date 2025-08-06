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
        draw_routes(g, out);

        draw_debug_overlay(g, out);
    }
}

fn visible_tiles(g: &G) -> impl Iterator<Item = (vec2i16, Tile)> {
    // 🪲 TODO: restrict to viewport
    g.tilemap.enumerate_all()
}

fn draw_tilemap(g: &G, out: &mut Out) {
    for (idx, mat) in visible_tiles(g) {
        out.push_sprite(L_TILES, mat.sprite(), idx.pos() - g.camera_pos);
    }
}

fn draw_debug_overlay(g: &G, out: &mut Out) {
    if g.debug.show_walkable {
        draw_walkalbe_overlay(g, out);
    }
}

fn draw_walkalbe_overlay(g: &G, out: &mut Out) {
    let color = RGBA::new(255, 0, 0, 100);
    for (idx, _) in visible_tiles(g) {
        if !g.is_walkable(idx) {
            let bounds = Bounds2D::from_pos_size(idx.pos(), TILE_VSIZE).translated(-g.camera_pos);
            out.push_rect(L_SPRITES + 1, Rectangle::new(bounds, color).with_fill(color));
        }
    }
}

fn draw_buildings(g: &G, out: &mut Out) {
    for building in &g.buildings {
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

fn draw_routes(g: &G, out: &mut Out) {
    for pawn in g.selected_pawns() {
        if !pawn.is_at_destination() {
            out.push_line(L_SPRITES, Line::new(pawn.center(), pawn.dest.pos() + TILE_ISIZE / 2).with_color(RGB::WHITE.with_alpha(128)).translated(-g.camera_pos));
        }
    }
}
