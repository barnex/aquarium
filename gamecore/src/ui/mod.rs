use crate::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct Ui {
}

impl Ui {
    pub fn new() -> Self {
        Self {}
    }
}


pub struct Palette {
    pos: vec2i,
    selected: Mat,
}

pub fn menu_ui(gs: &mut State, out: &mut Output) {
    // test

    // out.new_layer();
    //

    let pos = vec2(20, 10);
    let stride = TILE_SIZE + 1;
    let cols = 2;
    let size = vec2(cols * stride, 200);
    let bounds = Bounds2Di::new(pos, pos + size.as_());

    //writeln!(&mut out.debug, "bounds: {bounds:?}");

    out.push_rect(Rectangle::new(bounds, RGBA::BLACK).with_fill(RGBA::WHITE));

    let mut grid = GridLayout::new(bounds);

    for i in 0..Mat::NUM_MAT {
        let mat = Mat::try_from_primitive(i).unwrap();

        let pos = grid.layout(vec2(stride, stride));
        out.push_sprite(mat.sprite(), pos);
    }
}

struct GridLayout {
    bounds: Bounds2Di,
    cursor: vec2i,
}

impl GridLayout {
    pub fn new(bounds: Bounds2Di) -> Self {
        Self { bounds, cursor: bounds.min }
    }

    pub fn layout(&mut self, size: vec2u) -> vec2i {
        let size = size.as_i32();
        let result = self.cursor;

        self.cursor[0] += size.x();
        if self.cursor.x() >= self.bounds.max.x() {
            self.cursor[0] = self.bounds.min.x();
            self.cursor[1] += size.y();
        }

        result
    }
}
