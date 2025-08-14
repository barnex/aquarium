//! Draw the world.
use crate::prelude::*;

impl G {
    pub fn draw_world(&self, out: &mut Out) {
        debug_assert!(out.viewport_size != vec2::ZERO);
        // Note: ⚠️ UI already rendered (may consume input events)

        let g = self;
        draw_tilemap(g, out);
        draw_water(g, out);
        draw_buildings(g, out);
        draw_resources(g, out);
        draw_pawns(g, out);
        draw_cursor(g, out);
        draw_selection(g, out);

        draw_debug_overlay(g, out);
    }
}

pub(super) fn visible_tiles(g: &G) -> impl Iterator<Item = (vec2i16, Tile)> {
    // 🪲 TODO: restrict to viewport
    //g.tilemap.enumerate_all()

    let min = g.camera_pos.to_tile() - 1;
    let max = (g.camera_pos + g.viewport_size.as_i32()).to_tile() + 1;

    g.tilemap.enumerate_range(Bounds2D::new(min, max))
}

pub(super) fn visible_pawns(g: &G) -> impl Iterator<Item = &Pawn> {
    // 🪲 TODO: restrict to viewport
    g.pawns.iter()
}

fn draw_water(g: &G, out: &mut Out) {
    for (tile, mat) in visible_tiles(g) {
        if mat == Tile::Canal {
            let level = g.water_level_at(tile);
            if level != 0.0 {
                let a = linterp(0.0, 0.0, 100.0, 255.0, level).clamp(0.0, 255.0) as u8;
                let color = RGBA([0, 0, 255, a]);
                let bounds = Bounds2D::with_size(tile.pos(), TILE_VSIZE);
                out.draw_rect(g, L_WATER, Rectangle::new(bounds, RGBA::TRANSPARENT).with_fill(color));
            }
        }
    }
}

fn draw_tilemap(g: &G, out: &mut Out) {
    for (idx, mat) in visible_tiles(g) {
        out.draw_sprite_screen(L_TILES, mat.sprite(), idx.pos() - g.camera_pos);
    }
}

fn draw_buildings(g: &G, out: &mut Out) {
    for building in g.buildings.iter() {
        out.draw_sprite_screen(L_SPRITES, building.typ.sprite(), building.tile.pos() - g.camera_pos);
    }
}

fn draw_resources(g: &G, out: &mut Out) {
    for (tile, res) in g.resources.iter() {
        out.draw_sprite_screen(L_SPRITES, res.sprite(), tile.pos() - g.camera_pos);
    }
}

fn draw_pawns(g: &G, out: &mut Out) {
    for pawn in g.pawns.iter() {
        out.draw_sprite_screen(L_SPRITES, pawn.typ.sprite(), pawn.tile.pos() - g.camera_pos);
        if let Some(res) = pawn.cargo.get() {
            out.draw_sprite_screen(L_SPRITES + 1, res.sprite(), pawn.tile.pos() - g.camera_pos + vec2(0, 8));
        }
    }
}

// ↑
fn draw_cursor(g: &G, out: &mut Out) {
    let sprite = cursor_sprite(g);
    out.draw_sprite_screen(L_SPRITES, sprite, g.mouse_tile().pos() - g.camera_pos);
    out.draw_sprite_screen(L_SPRITES, sprite!("grid24"), g.mouse_tile().pos() - g.camera_pos);
}

fn cursor_sprite(g: &G) -> Sprite {
    match g.ui.active_tool {
        Tool::Tile(typ) => typ.sprite(),
        Tool::Pawn(typ) => typ.sprite(),
        Tool::Building(typ) => typ.sprite(),
        Tool::Resource(typ) => typ.sprite(),
        Tool::Pointer => match g.contextual_action {
            Action::Move => sprite!("target"),
            Action::Assign => sprite!("assign"),
            Action::None => sprite!("grid24"),
        },
    }
}

fn draw_selection(g: &G, out: &mut Out) -> Status {
    // 🖱️ Selection rectangle, if dragging mouse
    if let Some(start) = g.selection_start {
        let end = g.mouse_position_world();

        let min = start.zip_with(end, i32::min);
        let max = start.zip_with(end, i32::max);
        let sel = Bounds2D::new(min, max);

        out.push_rect_screen(L_SPRITES + 1, Rectangle::new(sel.translated(-g.camera_pos), RGBA::BLUE).with_fill(RGB::BLUE.with_alpha(64)));
    }

    // 🦀 Selected pawns
    for pawn in g.selected_pawns() {
        out.push_rect_screen(L_SPRITES + 1, Rectangle::new(pawn.bounds().translated(-g.camera_pos), RGBA::BLUE).with_fill(RGB::BLUE.with_alpha(64)));
    }
    OK
}
