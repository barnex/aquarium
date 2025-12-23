//! Draw the world.
use crate::prelude::*;

impl G {
    pub fn draw_world(&self, out: &mut Out) {
        debug_assert!(out.viewport_size != vec2::ZERO);

        out.draw_text_screen(L_TEXT, vec2(0, 0), &self.header_text);

        // Note: ⚠️ UI already rendered (may consume input events)

        let g = self;
        draw_tilemap(g, out);
        draw_water(g, out);
        draw_resources(g, out);
        visible_entities(g).for_each(|e| e.draw(g, out));
        //draw_entities(g, out)
        //self.entities().for_each(|e| e.draw(out));
        draw_cursor(g, out);
        draw_selection(g, out);
        self.effects.tick_and_draw(g, out);

        draw_debug_overlay(g, out);
    }
}

pub(super) fn visible_tiles(g: &G) -> impl Iterator<Item = (vec2i16, Tile)> {
    g._tilemap.enumerate_range(visible_tile_range(g))
}

pub(super) fn visible_tile_range(g: &G) -> Bounds2D<i16> {
    let min = g.camera_pos.to_tile() - 1;
    let max = (g.camera_pos + g.viewport_size.as_i32()).to_tile() + 1;
    Bounds2D::new(min, max)
}

pub(super) fn visible_pawns(g: &G) -> impl Iterator<Item = &Pawn> {
    let viewport = visible_tile_range(g);
    g.pawns().filter(move |p| viewport.contains(p.tile.get()))
}

pub(super) fn visible_entities(g: &G) -> impl Iterator<Item = Entity> {
    let viewport = visible_tile_range(g);
    g.entities().filter(move |p| viewport.contains(p.tile()))
}

fn draw_water(g: &G, out: &mut Out) {
    for (tile, _) in visible_tiles(g) {
        // draw water even if not on canal, so we see it in case of issues.
        if g.water.h.contains_key(&tile) || g.water.p.contains_key(&tile) {
            let level = g.water_level_at(tile);
            // level
            let (r, b) = if level > 0.0 { (0, 255) } else { (255, 0) };
            let a = (level * 255.0).clamp(0.0, 255.0) as u8;
            let color = RGBA([r, 0, b, a]);
            let bounds = Bounds2D::with_size(tile.pos(), TILE_VSIZE);
            out.draw_rect(L_WATER, Rectangle::new(bounds, RGBA::TRANSPARENT).with_fill(color));

            // speed arrow
            let speed = g.water.water_speed_at(tile);
            let mid = (tile.pos() + TILE_VSIZE / 2).as_f32();
            let start = mid - speed * (TILE_SIZE / 2) as f32;
            let end = mid + speed * (TILE_SIZE / 2) as f32;
            let arrow = Line::new(start.as_i32(), end.as_i32()).with_color(RGBA::RED).with_width(2);
            out.draw_line(L_WATER + 1, arrow);
            let bud = Rectangle::new(Bounds2D::new(start.as_i32() - 2, start.as_i32() + 2), RGBA::RED);
            out.draw_rect(L_WATER + 1, bud);
        }
    }
}

fn draw_tilemap(g: &G, out: &mut Out) {
    for (idx, mat) in visible_tiles(g) {
        out.draw_sprite(L_TILES, mat.sprite(), idx.pos());
    }
}

fn draw_resources(g: &G, out: &mut Out) {
    for (tile, res) in g.resources.iter() {
        out.draw_sprite(L_SPRITES, res.sprite(), tile.pos());
    }
}

// ↑
fn draw_cursor(g: &G, out: &mut Out) {
    let sprite = cursor_sprite(g);
    out.draw_sprite(L_SPRITES, sprite, g.mouse_tile().pos());
    out.draw_sprite(L_SPRITES, sprite!("grid24"), g.mouse_tile().pos());
}

fn cursor_sprite(g: &G) -> Sprite {
    match g.ui.active_tool {
        Tool::Tile(typ) => typ.sprite(),
        Tool::Pawn2(typ, team) => Pawn::new(typ, default(), team).sprite(), // <<< TODO: proper icon
        Tool::Building(typ) => typ.sprite(),
        Tool::Resource(typ) => typ.sprite(),
        Tool::Pointer => match g.contextual_action {
            Action::Move => sprite!("target"),
            Action::Assign => sprite!("assign"),
            Action::None => sprite!("grid24"),
        },
        Tool::WaterBucket => sprite!("droplet"),
    }
}

fn draw_selection(g: &G, out: &mut Out) -> Status {
    // 🖱️ Selection rectangle, if dragging mouse
    if let Some(start) = g.selection_start {
        let end = g.mouse_position_world();

        let min = start.zip_with(end, i32::min);
        let max = start.zip_with(end, i32::max);
        let sel = Bounds2D::new(min, max);

        out.draw_rect(L_SPRITES + 1, Rectangle::new(sel, RGBA::BLUE).with_fill(RGB::BLUE.with_alpha(64)));
    }

    // 🦀 Selected pawns
    for entity in g.selected_entities() {
        out.draw_rect(L_SPRITES + 1, Rectangle::new(entity.bounds().map(|v| (v as i32) * TILE_ISIZE), RGBA::BLUE).with_fill(RGB::BLUE.with_alpha(64)));
    }
    OK
}
