//! Draw the world.
use crate::prelude::*;

impl G {
    pub fn draw_world(&self, out: &mut Output) {
        debug_assert!(self.viewport_size != vec2::ZERO);
        // Note: ⚠️ UI already rendered (may consume input events)

        let g = self;
        draw_tilemap(g, out);
        draw_buildings(g, out);
        draw_pawns(g, out);
        draw_cursor(g, out);
        draw_selection(g, out);
        draw_routes(g, out);
    }
}

pub fn draw_tilemap(g: &G, out: &mut Output) {
    for (idx, mat) in g.tilemap.enumerate_all() {
        out.push_sprite(L_TILES, mat.sprite(), idx.pos() - g.camera_pos);
    }
}

pub fn draw_buildings(g: &G, out: &mut Output) {
    for building in &g.buildings {
        out.push_sprite(L_SPRITES, building.typ.sprite(), building.tile * TILE_ISIZE - g.camera_pos);
    }
}

fn draw_pawns(g: &G, out: &mut Output) {
    for pawn in g.pawns.iter() {
        out.push_sprite(L_SPRITES, pawn.typ.sprite(), pawn.tile.pos() - g.camera_pos);
    }
}

fn draw_cursor(g: &G, out: &mut Output) {
    let sprite = match g.ui.active_tool {
        Tool::Pointer => sprite!("grid24"),
        Tool::Tile(typ) => typ.sprite(),
        Tool::Pawn(typ) => typ.sprite(),
    };
    out.push_sprite(L_SPRITES, sprite, g.mouse_tile().pos() - g.camera_pos);
    out.push_sprite(L_SPRITES, sprite!("grid24"), g.mouse_tile().pos() - g.camera_pos);
}

fn draw_selection(g: &G, out: &mut Output) -> Option<()> {
    if let Some(start) = g.selection_start {
        let end = g.mouse_position_world();

        let min = start.zip_with(end, i32::min);
        let max = start.zip_with(end, i32::max);
        let sel = Bounds2D::new(min, max);

        out.push_rect(L_SPRITES, Rectangle::new(sel.translated(-g.camera_pos), RGBA::BLUE).with_fill(RGB::BLUE.with_alpha(64)));
    }

    for sel in g.selected.iter().filter_map(|&id| g.pawn(id)) {
        out.push_rect(L_SPRITES, Rectangle::new(sel.bounds().translated(-g.camera_pos), RGBA::BLUE).with_fill(RGB::BLUE.with_alpha(64)));
    }
    OK
}

fn draw_routes(g: &G, out: &mut Output) {
    for pawn in g.selected.iter().filter_map(|&id| g.pawn(id)) {
        if !pawn.is_at_destination() {
            out.push_line(L_SPRITES, Line::new(pawn.center(), pawn.dest.pos() + TILE_ISIZE / 2).with_color(RGB::WHITE.with_alpha(128)).translated(-g.camera_pos));
        }
    }
}
