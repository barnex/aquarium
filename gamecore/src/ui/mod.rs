use crate::prelude::*;

pub fn draw_menus(out: &mut Output) {
    // test
    //out.push_line(Line::new(vec2(0, 0), vec2(30, 20)));
    //out.push_rect(Rectangle::new(Bounds::new(vec2(10, 10), vec2(74, 138)), RGBA::BLACK).with_fill(RGBA::WHITE));
    out.push_rect(Rectangle::new(Bounds::new(vec2(10, 10), vec2(74, 138)), RGBA::BLACK));
	
	for i in 0..Mat::NUM_MAT{
		let mat = Mat::try_from_primitive(i).unwrap();
		
		out.push_sprite(mat.sprite(), vec2(10, 10+(i as i32) * TILE_ISIZE));
	}
	
}
