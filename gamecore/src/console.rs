use crate::prelude::*;
use std::mem::take;

#[derive(Default, Serialize, Deserialize)]
pub struct Console {
    // 📺 currently entering text in console ?
    pub mode: bool,
    // 📺 line of text currently being typed into console.
    pub linebuffer: String,
}

const BACKSPACE: char = '\u{08}';
const BACKSPACE_MAC: char = '\u{7F}';
const ENTER: char = '\u{0D}';
const ESCAPE: char = '\u{1B}';

impl G {
    // 🪲 TODO: JS: addEventListener("input") + macroquad equivalent to get actual characters + no keymapping
    pub(crate) fn tick_console(&mut self) {
        if self.inputs.just_pressed(K_CLI) {
            toggle(&mut self.console.mode)
        }

        if !self.console.mode {
            return;
        }

        for chr in self.inputs.input_characters().chars() {
            match chr {
                BACKSPACE | BACKSPACE_MAC => drop(self.console.linebuffer.pop()), // backspace (linux, windows | mac)
                ENTER => self.commands.push_back(take(&mut self.console.linebuffer)),
                ESCAPE => self.console.mode = false,
                chr if !chr.is_ascii_control() => self.console.linebuffer.push(chr),
                _ => (),
            }
        }
    }

    pub(crate) fn draw_console(&self, out: &mut Out) {
        if !self.console.mode {
            return;
        }

        let layer = L_CLI;
        const CONSOLE_BG: RGBA = RGBA([0, 0, 0, 184]);
        out.draw_rect_screen(layer, Rectangle::new(Bounds2D::new(vec2(0, 0), self.viewport_size.as_()), RGBA::TRANSPARENT).with_fill(CONSOLE_BG));
        draw_text(out, layer, vec2(0, 0), &(">".to_string() + &self.console.linebuffer + "_"));
    }
}
