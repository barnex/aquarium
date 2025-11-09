//! Render game output (scenegraph) on a HtmlCanvasElement via web_sys (Javascript) API.
//!
//! Alternative renderer: see headless_renderer.rs.
use crate::*;

pub(crate) fn draw(canvas: &HtmlCanvasElement, ctx: &CanvasRenderingContext2d, res: &mut Resources, out: &Out) {
    res.poll(); // 👈 Allow newly loaded images to be used.

    ctx.set_image_smoothing_enabled(false); // crisp, pixellated sprites
    ctx.clear_rect(0.0, 0.0, canvas.width().as_(), canvas.height().as_());

    // Draw layers starting from 0 for correct Z-ordering.
    for Layer { sprites, lines, rectangles } in &out.layers {
        // ▭ rectangles
        for rect in rectangles {
            if rect.fill != RGBA::TRANSPARENT {
                ctx.set_fill_style_str(&rect.fill.hex());
                ctx.fill_rect(
                    //_
                    rect.bounds.min.x().as_(),
                    rect.bounds.min.y().as_(),
                    rect.bounds.size().x().as_(),
                    rect.bounds.size().y().as_(),
                );
            }

            if rect.stroke != RGBA::TRANSPARENT {
                ctx.set_stroke_style_str(&rect.stroke.hex());
                ctx.set_line_width(1.0);
                // 👇 HTML Canvas aligns strokes (but not fills) to the edges of pixels instead of to the center.
                // Offset by half a pixel to align pixel-perfect.
                ctx.stroke_rect(
                    //_
                    (rect.bounds.min.x() as f64) + 0.5,
                    (rect.bounds.min.y() as f64) + 0.5,
                    (rect.bounds.size().x() as f64) - 1.0,
                    (rect.bounds.size().y() as f64) - 1.0,
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

                let src_size = match cmd.src_pos {
                    None => vec2(bitmap.width(), bitmap.height()),
                    Some(_) => dst_size,
                };

                let source = match cmd.src_pos {
                    None => vec2d(0.0, 0.0),
                    Some(src) => src.as_f64(),
                };

                ctx.draw_image_with_image_bitmap_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                    bitmap,
                    source.x(),         // source x
                    source.y(),         // source y
                    src_size.x().as_(), // source width
                    src_size.y().as_(), // source height
                    cmd.pos.x().as_(),  // dest x
                    cmd.pos.y().as_(),  // dest y
                    dst_size.x().as_(), // dest width
                    dst_size.y().as_(), // dest height
                )
                .expect("draw");
            }
        }

        // ╱ lines
        for line in lines {
            // TODO: need 0.5 pix offset
            ctx.begin_path();
            ctx.set_stroke_style_str(&line.color.hex());
            ctx.set_line_width(line.width.as_());
            ctx.move_to(line.start.x().as_(), line.start.y().as_());
            ctx.line_to(line.end.x().as_(), line.end.y().as_());
            ctx.stroke();
        }
    }

    //graphics_postprocessing::bloom(canvas, ctx);
    graphics_postprocessing::vignette(canvas, ctx);
}
