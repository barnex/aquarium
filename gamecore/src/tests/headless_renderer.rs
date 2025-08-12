use crate::*;
use anyhow::Result;
use core_util::*;
use num_traits::AsPrimitive as _;
use std::path::Path;
use tiny_skia::{Color, FillRule, Paint, PathBuilder, Pixmap, PixmapPaint, Rect, Stroke, Transform};
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
            let fill_paint = Paint::default().with(|fill_paint| {
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

            let stroke = Stroke::default();
            //stroke.width = r.stroke_width; // border thickness in device pixels
            //stroke.line_join = LineJoin::Round;
            //stroke.line_cap = LineCap::Round;

            // Stroke the path (draw the border)
            canvas.stroke_path(&path, &stroke_paint, &stroke, Transform::identity(), None);
        }

        // 🦀 sprites
        for cmd in sprites {
            let bitmap = res_get(&cmd.sprite);
            // 🪲 TODO: scale
            let dst_size = match cmd.dst_size {
                None => vec2(bitmap.width(), bitmap.height()),
                Some(dst_size) => dst_size.map(|v| v.get().as_()),
            };

            canvas.draw_pixmap(cmd.pos.x(), cmd.pos.y(), bitmap.as_ref(), &PixmapPaint::default(), Transform::identity(), None);

            // 🪲 TODO: scale
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
            let mut pb = PathBuilder::new();
            let (x1, y1) = line.start.as_f32().into();
            let (x2, y2) = line.end.as_f32().into();
            pb.move_to(x1, y1);
            pb.line_to(x2, y2);
            let path = pb.finish().unwrap();

            let paint = Paint::default().with(|paint| {
                let [r, b, g, a] = line.color.into();
                paint.set_color_rgba8(r, g, b, a); // red line
                paint.anti_alias = true;
            });

            let stroke = Stroke::default().with(|stroke| {
                stroke.width = line.width.as_();
            });

            canvas.stroke_path(&path, &paint, &stroke, Transform::identity(), None);
        }
    }

    canvas.save_png(file)?;
    Ok(())
}
