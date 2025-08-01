use super::internal::*;

#[derive(Serialize, Deserialize)]
pub(super) struct Palette {
    pub pos: vec2i,
    pub rows: u32,
    pub cols: u32,
    pub button_size: vec2u,
    pub margin: u32,
}

impl Palette {
    pub(super) fn ui<T>(&self, inputs: &mut Inputs, out: &mut Output, selection: &mut T, options: impl Iterator<Item = (T, Sprite)>)
    where
        T: PartialEq + Copy, // TODO: don't require copy bound
    {
        let size = vec2(self.cols, self.rows) * (self.button_size + self.margin) + self.margin;
        let rect = Bounds2D::with_size(self.pos, size.as_i32());

        out.push_rect(L_UI_BG, Rectangle::new(rect, RGBA(vec4u8(128, 128, 128, 128))).with_fill(RGBA::WHITE));

        let (mut row, mut col) = (0, 0);
        for (option, sprite) in options {
            let pos = (vec2(col, row) * (self.button_size + self.margin) + self.margin).as_i32() + self.pos;
            out.push_sprite(L_UI, sprite, pos);

            if inputs.just_pressed(K_MOUSE1) {
                if Bounds2D::with_size(pos, self.button_size.as_i32()).contains(inputs.mouse_position()) {
                    *selection = option;
                }
            }

            if selection == &option {
                let min = pos - (self.margin as i32) + 1;
                let max = pos + self.button_size.as_i32() + (self.margin as i32) - 1;
                out.push_rect(L_UI, Rectangle::new(Bounds2D::new(min, max), RGBA::TRANSPARENT).with_fill(RGBA::RED));
            }

            col += 1;
            if col == self.cols {
                col = 0;
                row += 1
            }
            if row >= self.rows {
                break;
            }
        }

        if rect.contains(inputs.mouse_position()) {
            inputs.consume(K_MOUSE1);
            inputs.consume(K_MOUSE2);
        }
    }
}
