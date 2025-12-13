use crate::*;
use num_traits::AsPrimitive as _;

/// Draw scenegraph via macroquad.
pub(crate) fn mq_draw(res: &mut Resources, out: &Out) {
    res.poll(); // 👈 Allow newly loaded images to be used.

    //mq::clear_background(mq::LIGHTGRAY);

    // Draw layers starting from 0 for correct Z-ordering.
    for Layer { sprites, lines, rectangles } in &out.layers {
        // ▭ rectangles
        for rect in rectangles {
            if rect.fill != RGBA::TRANSPARENT {
                mq::draw_rectangle(
                    //_
                    rect.bounds.min.x().as_(),
                    rect.bounds.min.y().as_(),
                    rect.bounds.size().x().as_(),
                    rect.bounds.size().y().as_(),
                    mq_color(rect.fill),
                );
            }

            if rect.stroke != RGBA::TRANSPARENT {
                let line_width = 2.0; // 👈 macroquad seems buggy with width 1.0 (draws only half the lines).
                mq::draw_rectangle_lines(
                    //_
                    rect.bounds.min.x().as_(),
                    rect.bounds.min.y().as_(),
                    rect.bounds.size().x().as_(),
                    rect.bounds.size().y().as_(),
                    line_width,
                    mq_color(rect.stroke),
                );
            }
        }

        // 🦀 sprites
        for cmd in sprites {
            if let Some(bitmap) = res.get(&cmd.sprite) {
                let dst_size = match cmd.dst_size {
                    None => vec2(bitmap.width(), bitmap.height()),
                    Some(dst_size) => dst_size.map(|v| v.get().as_()),
                };

                let source = match cmd.src_pos {
                    None => None,
                    Some(src) => Some(mq::Rect::new(src.x().as_(), src.y().as_(), dst_size.x(), dst_size.y())),
                };

                mq::draw_texture_ex(
                    bitmap,
                    cmd.pos.x().as_(),
                    cmd.pos.y().as_(),
                    mq::WHITE,
                    mq::DrawTextureParams {
                        dest_size: Some(mq::Vec2::new(dst_size.x(), dst_size.y())),
                        source,
                        rotation: cmd.rot,
                        ..Default::default()
                    },
                );

                //ctx.draw_image_with_image_bitmap_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                //    bitmap,
                //    0.0,                   // source x
                //    0.0,                   // source y
                //    bitmap.width().as_(),  // source width
                //    bitmap.height().as_(), // source height
                //    cmd.pos.x().as_(),     // dest x
                //    cmd.pos.y().as_(),     // dest y
                //    dst_size.x().as_(),    // dest width
                //    dst_size.y().as_(),    // dest height
                //)
                //.expect("draw");
            }
        }

        // ╱ lines
        for line in lines {
            mq::draw_line(
                //_
                line.start.x().as_(),
                line.start.y().as_(),
                line.end.x().as_(),
                line.end.y().as_(),
                line.width.as_(),
                mq_color(line.color),
            );
        }
    }
}

fn mq_color(c: RGBA) -> mq::Color {
    let RGBA([r, g, b, a]) = c;
    mq::Color::from_rgba(r, g, b, a)
}
