//! Draw the world.
use crate::prelude::*;

impl G {
    pub fn draw_world(&self, out: &mut Output) {
        debug_assert!(self.viewport_size != vec2::ZERO);
        // Note: ⚠️ UI already rendered (may consume input events)

        self.draw_tilemap(out);
        self.draw_buildings(out);
        self.draw_pawns(out);
        self.draw_cursor(out);
        self.draw_selection(out);
        self.draw_routes(out);
    }

    pub fn draw_tilemap(&self, out: &mut Output) {
        for (idx, mat) in self.tilemap.enumerate_all() {
            out.push_sprite(L_TILES, mat.sprite(), idx.pos() - self.camera_pos);
        }
    }

    pub fn draw_buildings(&self, out: &mut Output) {
        for building in &self.buildings {
            out.push_sprite(L_SPRITES, building.typ.sprite(), building.tile * TILE_ISIZE - self.camera_pos);
        }
    }

    fn draw_pawns(&self, out: &mut Output) {
        for pawn in self.pawns.iter() {
            out.push_sprite(L_SPRITES, pawn.typ.sprite(), pawn.tile.pos() - self.camera_pos);
        }
    }

    fn draw_cursor(&self, out: &mut Output) {
        let sprite = match self.ui.active_tool {
            Tool::Pointer => sprite!("grid24"),
            Tool::Tile(typ) => typ.sprite(),
            Tool::Pawn(typ) => typ.sprite(),
        };
        out.push_sprite(L_SPRITES, sprite, self.mouse_tile().pos() - self.camera_pos);
        out.push_sprite(L_SPRITES, sprite!("grid24"), self.mouse_tile().pos() - self.camera_pos);
    }

    fn draw_selection(&self, out: &mut Output) -> Option<()> {
        let sel = self.pawns.get(self.selected.get()?)?;
        out.push_rect(L_SPRITES, Rectangle::new(sel.bounds().translated(-self.camera_pos), RGBA::BLUE).with_fill(RGB::BLUE.with_alpha(64)));
        OK
    }

    fn draw_routes(&self, out: &mut Output) -> Option<()> {
        let pawn = self.pawns.get(self.selected.get()?)?;
		if !pawn.is_at_destination(){
			out.push_line(L_SPRITES, Line::new(pawn.center(), pawn.dest.pos() + TILE_ISIZE/2).with_color(RGB::WHITE.with_alpha(128)).translated(-self.camera_pos));	
		}

        OK
    }
}
