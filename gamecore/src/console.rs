use crate::prelude::*;
use std::mem::take;

impl G {
    // 🪲 TODO: JS: addEventListener("input") + macroquad equivalent to get actual characters + no keymapping
    pub(crate) fn handle_console_mode(&mut self) {
        for key in self.inputs.iter_just_pressed() {
            match key {
                K_BACKSPACE => drop(self.console_linebuffer.pop()), // backspace (linux, windows | mac)
                K_ESC => self.console_mode = false,
                K_ENTER => {
                    self.commands.push_back(take(&mut self.console_linebuffer));
                }
                key if key.len() == 1 => {
                    if let Some(chr) = char::from_u32(key[0] as u32) {
                        self.console_linebuffer.push(chr)
                    }
                }
                _ => (),
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
