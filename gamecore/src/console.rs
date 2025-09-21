use crate::prelude::*;
use std::mem::take;

const BACKSPACE: char = '\u{08}';
const BACKSPACE_MAC: char = '\u{7F}';
const ENTER: char = '\u{0D}';
const ESCAPE: char = '\u{1B}';

impl G {
    // 🪲 TODO: JS: addEventListener("input") + macroquad equivalent to get actual characters + no keymapping
    pub(crate) fn handle_console_mode(&mut self) {
        for chr in self.inputs.input_characters().chars() {
            match chr {
                BACKSPACE | BACKSPACE_MAC => drop(self.console_linebuffer.pop()), // backspace (linux, windows | mac)
                ENTER => self.commands.push_back(take(&mut self.console_linebuffer)),
                ESCAPE => self.console_mode = false,
                chr => self.console_linebuffer.push(chr),
            }
        }
    }

    pub(crate) fn draw_console_mode(&self, out: &mut Out) {
        if !self.console_mode {
            return;
        }

        let layer = L_CLI;

        out.draw_rect_screen(layer, Rectangle::new(Bounds2D::new(vec2(0, 0), self.viewport_size.as_()), RGBA::TRANSPARENT).with_fill(RGBA([0, 0, 0, 220])));
        draw_text(out, layer, vec2(0, 0), &(">".to_string() + &self.console_linebuffer));
    }
}
