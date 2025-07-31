use super::internal::*;

pub struct Ui {
    pub tile_picker: Option<Mat>,
}

impl Ui {
    pub fn new() -> Self {
        Self { tile_picker: Some(Mat::Block) }
    }

    pub fn update_and_draw(&mut self, inputs: &mut Inputs, out: &mut Output) {
        self.tile_palette_ui(inputs, out)
    }

    fn tile_palette_ui(&mut self, inputs: &mut Inputs, out: &mut Output) {
        let margin = 3;
        Palette {
            pos: vec2(4, 2),
            cols: 2,
            rows: 8, // ?
            button_size: vec2(TILE_SIZE, TILE_SIZE),
            margin,
        }
        .ui(inputs, out, &mut self.tile_picker, (0..Mat::NUM_MAT).map(|i| Mat::try_from_primitive(i).unwrap()).map(|mat| (mat, mat.sprite())));
    }
}

impl Default for Ui {
    fn default() -> Self {
        Self::new()
    }
}
