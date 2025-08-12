use crate::*;
use anyhow::Result;
use core_util::*;
use num_traits::AsPrimitive as _;
use std::{
    env,
    path::{Path, PathBuf},
};
use tiny_skia::{Color, FillRule, LineCap, LineJoin, Paint, PathBuilder, Pixmap, PixmapPaint, Rect, Stroke, Transform};
use vector::*;

fn res_get(sprite: &Sprite) -> Pixmap {
    let file = format!("../assets/{}.png", sprite.file.as_str());
    Pixmap::load_png(&file).expect(&format!("load pixmap {file}"))
}

pub fn render_headless(out: &Out, file: impl AsRef<Path>) -> Result<()> {
    let file = file.as_ref();
    let (w, h) = out.viewport_size.into();
    let mut canvas = Pixmap::new(w, h).expect("skia Pixmap");
    canvas.fill(Color::from_rgba8(255, 255, 255, 255));

    // Draw layers starting from 0 for correct Z-ordering.
    for Layer { sprites, lines, rectangles } in &out.layers {
        // ▭ rectangles
        for r in rectangles {
            // rectangle geometry (left, top, right, bottom)
            let (lt, rb) = (r.bounds.min.as_f32(), r.bounds.max.as_f32());
            let rect = Rect::from_ltrb(lt.x(), lt.y(), rb.x(), rb.y()).unwrap();

            // build a Path from the rect
            let path = PathBuilder::from_rect(rect);

            // --- Fill paint ---
            let mut fill_paint = Paint::default().with(|fill_paint|{
            let [r, g, b, a] = r.fill.into();
            fill_paint.set_color_rgba8(r, g, b, a);
            fill_paint.anti_alias = true;
            });

            // Fill the path
            canvas.fill_path(&path, &fill_paint, FillRule::Winding, Transform::identity(), None);

            // --- Stroke paint & stroke settings ---
            let mut stroke_paint = Paint::default();
            stroke_paint.set_color_rgba8(10, 50, 120, 255); // dark blue border
            stroke_paint.anti_alias = true;

            let mut stroke = Stroke::default();
            //stroke.width = r.stroke_width; // border thickness in device pixels
            //stroke.line_join = LineJoin::Round;
            //stroke.line_cap = LineCap::Round;

            // Stroke the path (draw the border)
            canvas.stroke_path(&path, &stroke_paint, &stroke, Transform::identity(), None);

            //if rect.fill != RGBA::TRANSPARENT {
            //    //ctx.set_fill_style_str(&rect.fill.hex());
            //    //ctx.fill_rect(rect.bounds.min.x().as_(), rect.bounds.min.y().as_(), rect.bounds.size().x().as_(), rect.bounds.size().y().as_());
            //}

            //if rect.stroke != RGBA::TRANSPARENT {
            //    //ctx.set_stroke_style_str(&rect.stroke.hex());
            //    //ctx.set_line_width(1.0);
            //    // 👇 HTML Canvas aligns strokes (but not fills) to the edges of pixels instead of to the center.
            //    // Offset by half a pixel to align pixel-perfect.
            //    //ctx.stroke_rect((rect.bounds.min.x() as f64) + 0.5, (rect.bounds.min.y() as f64) + 0.5, (rect.bounds.size().x() as f64) - 1.0, (rect.bounds.size().y() as f64) - 1.0);
            //}
        }

        // 🦀 sprites
        for cmd in sprites {
            let bitmap = res_get(&cmd.sprite);
            let dst_size = match cmd.dst_size {
                None => vec2(bitmap.width(), bitmap.height()),
                Some(dst_size) => dst_size.map(|v| v.get().as_()),
            };

            canvas.draw_pixmap(cmd.pos.x(), cmd.pos.y(), bitmap.as_ref(), &PixmapPaint::default(), Transform::identity(), None);

            // ctx.draw_image_with_image_bitmap_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
            //     bitmap,
            //     0.0,                   // source x
            //     0.0,                   // source y
            //     bitmap.width().as_(),  // source width
            //     bitmap.height().as_(), // source height
            //     cmd.pos.x().as_(),     // dest x
            //     cmd.pos.y().as_(),     // dest y
            //     dst_size.x().as_(),    // dest width
            //     dst_size.y().as_(),    // dest height
            // )
            // .expect("draw");
        }

        // ╱ lines
        for line in lines {
            //ctx.begin_path();
            //ctx.set_stroke_style_str(&line.color.hex());
            //ctx.set_line_width(line.width.as_());
            //ctx.move_to(line.start.x().as_(), line.start.y().as_());
            //ctx.line_to(line.end.x().as_(), line.end.y().as_());
            //ctx.stroke();
        }
    }

    // let sprite = Pixmap::load_png("mysprite.png")?;
    // canvas.draw_pixmap(
    //     100, // x
    //     50,  // y
    //     sprite.as_ref(),
    //     &PixmapPaint::default(),
    //     Transform::identity(),
    //     None,
    // );

    // // Save the resulting image as a PNG
    canvas.save_png(file)?;

    Ok(())
}
