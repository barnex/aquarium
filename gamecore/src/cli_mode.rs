use crate::prelude::*;

impl G {
    pub(crate) fn handle_cli_mode(&mut self) {}

    pub(crate) fn draw_cli_mode(&self, out: &mut Out) {
        if !self.cli_mode {
            return;
        }

		let layer = L_CLI;

        out.draw_rect_screen(layer, Rectangle::new(Bounds2D::new(vec2(0, 0), self.viewport_size.as_()), RGBA::TRANSPARENT).with_fill(RGBA([0, 0, 0, 220])));
        draw_text(out, layer, vec2(0, 0), ">");
    }
}
