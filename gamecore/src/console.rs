use crate::prelude::*;
use std::mem::take;

/// 📺 Game text console. Type game commands & cheats.
#[derive(Default, Serialize, Deserialize)]
pub struct Console {
    /// Is console currently shown? This diverts all input into the console, not actual game.
    pub active: bool,

    /// line of text currently being typed into console.
    pub input_buffer: String,

    pub output: CDeque<String>,
}

const BACKSPACE: char = '\u{08}';
const BACKSPACE_MAC: char = '\u{7F}';
const ENTER: char = '\u{0D}';
const ESCAPE: char = '\u{1B}';

impl Console {
    const MAX_SCROLLBACK: usize = 64;

    pub fn push_output(&self, v: impl Into<String>) {
        let v = v.into();
        self.output.push_back(v);
        while self.output.len() > Self::MAX_SCROLLBACK {
            self.output.pop_front();
        }
    }
}

impl G {
    pub fn print_to_console(&self, v: impl AsRef<str>) {
        let v = v.as_ref();
        for line in v.lines() {
            self.console.output.push_back(line.to_owned());
            while self.console.output.len() > Console::MAX_SCROLLBACK {
                self.console.output.pop_front();
            }
        }
    }

    // 🪲 TODO: JS: addEventListener("input") + macroquad equivalent to get actual characters + no keymapping
    pub(crate) fn tick_console(&mut self) {
        if self.inputs.just_pressed(K_CLI) {
            toggle(&mut self.console.active)
        }

        if !self.console.active {
            return;
        }

        for chr in self.inputs.input_characters().chars() {
            match chr {
                BACKSPACE | BACKSPACE_MAC => drop(self.console.input_buffer.pop()), // backspace (linux, windows | mac)
                ENTER => self.commands.push_back(take(&mut self.console.input_buffer)),
                //ENTER => self.console.output.push_back(take(&mut self.console.input_buffer)),
                ESCAPE => self.console.active = false,
                chr if !chr.is_ascii_control() => self.console.input_buffer.push(chr),
                _ => (),
            }
        }
    }

    pub(crate) fn draw_console(&self, out: &mut Out) {
        if !self.console.active {
            return;
        }

        let layer = L_CLI;
        const CONSOLE_BG: RGBA = RGBA([0, 0, 0, 184]);

        //let buffer_height = text_height

        let text = ">".to_string() + &self.console.input_buffer + "_";
        let screen_size = (self.viewport_size / EMBEDDED_CHAR_SIZE.as_u32()).as_u16();

        let buffer_height = text_height_lines(&text, screen_size.x());

        let mut y = (screen_size.y() - buffer_height) * (EMBEDDED_CHAR_SIZE.y() as u16);

        out.draw_rect_screen(layer, Rectangle::new(Bounds2D::new(vec2(0, 0), self.viewport_size.as_()), RGBA::TRANSPARENT).with_fill(CONSOLE_BG));
        draw_text(out, layer, vec2(0, y as i32), &text);

        let mut i = self.console.output.len();
        while i > 0 && y > 0 {
            i -= 1;
            if let Some(line) = self.console.output.get(i) {
                y -= text_height_lines(&line, screen_size.x()) * (EMBEDDED_CHAR_SIZE.y() as u16);
                draw_text(out, layer, vec2(0, y as i32), &line);
            }
        }
    }
}

fn text_height_lines(txt: &str, screen_width_lines: u16) -> u16 {
    let mut height = 0;
    for line in txt.lines() {
        height += line.len() as u16 / screen_width_lines + 1;
    }
    height
}
