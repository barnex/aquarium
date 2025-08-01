use std::iter;

use super::internal::*;

pub struct Ui {
    pub active_tool: Tool,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Tool {
    Pointer,
    Tile(Tile),
    Pawn(PawnTyp),
}

impl Ui {
    pub fn new() -> Self {
        Self { active_tool: Tool::Pointer }
    }

    pub fn update_and_draw(&mut self, inputs: &mut Inputs, out: &mut Output) {
        self.tile_picker_ui(inputs, out)
    }

    fn tile_picker_ui(&mut self, inputs: &mut Inputs, out: &mut Output) {
        let options = iter::once((Tool::Pointer, sprite!("pointer"))) //_
            .chain(Tile::all().map(|typ| (Tool::Tile(typ), typ.sprite())))
            .chain(PawnTyp::all().map(|typ| (Tool::Pawn(typ), typ.sprite())));

        let margin = 3;
        Palette {
            pos: vec2(2, 120),
            cols: 2,
            rows: 8,
            button_size: vec2(TILE_SIZE, TILE_SIZE),
            margin,
        }
        .ui(inputs, out, &mut self.active_tool, options);
    }
}

impl Default for Ui {
    fn default() -> Self {
        Self::new()
    }
}
