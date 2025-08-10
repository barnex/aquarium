use std::iter;

use super::internal::*;

pub struct Ui {
    pub hidden: bool,
    pub active_tool: Tool,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Tool {
    Pointer,
    Tile(Tile),
    Pawn(PawnTyp),
    Building(BuildingTyp),
    Resource(ResourceTyp),
}

impl Ui {
    pub fn new() -> Self {
        Self { hidden: false, active_tool: Tool::Pointer }
    }

    pub fn update_and_draw(&mut self, inputs: &mut Inputs, out: &mut Out) {
        if self.hidden {
            return;
        }

        self.menu_ui(inputs, out);

        // Right-click on map deselects and switches to pointer.
        if inputs.just_pressed(K_MOUSE2) {
            self.active_tool = Tool::Pointer;
        }
    }

    fn menu_ui(&mut self, inputs: &mut Inputs, out: &mut Out) {
        let buttons = iter::once((Tool::Pointer, sprite!("pointer"))) //_
            .chain(Tile::all().map(|typ| (Tool::Tile(typ), typ.sprite())))
            .chain(PawnTyp::all().map(|typ| (Tool::Pawn(typ), typ.sprite())))
            .chain(BuildingTyp::all().map(|typ| (Tool::Building(typ), typ.sprite())))
            .chain(ResourceTyp::all().map(|typ| (Tool::Resource(typ), typ.sprite())));

        let margin = 3;
        Palette {
            pos: vec2(2, 120),
            cols: 2,
            rows: 8,
            button_size: vec2(TILE_SIZE, TILE_SIZE),
            margin,
        }
        .ui(inputs, out, &mut self.active_tool, buttons);
    }
}

impl Default for Ui {
    fn default() -> Self {
        Self::new()
    }
}
