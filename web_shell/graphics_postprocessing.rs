use crate::*;

pub fn inverse_bloom(canvas: &HtmlCanvasElement, context: &CanvasRenderingContext2d) {
    if is_safari() {
        // Safari does not allow dst == src canvas.
        // Can't be bothered to work around this.
        return;
    }

    // --- Invert colors ---
    let width = canvas.width() as f64;
    let height = canvas.height() as f64;
    context.save();
    context.set_global_composite_operation("difference").unwrap();
    context.set_fill_style_str("white");
    context.fill_rect(0.0, 0.0, width, height);
    context.restore();

    // --- Bloom effect on inverted colors ---
    context.save();
    context.set_global_composite_operation("lighter").unwrap();
    context.set_filter("blur(4px)");
    context.draw_image_with_html_canvas_element(&canvas, 0.0, 0.0).unwrap();
    context.restore();

    // --- Invert back ---
    context.save();
    context.set_global_composite_operation("difference").unwrap();
    context.set_fill_style_str("white");
    context.fill_rect(0.0, 0.0, width, height);
    context.restore();
	
}

pub fn bloom(canvas: &HtmlCanvasElement, context: &CanvasRenderingContext2d) {
    if is_safari() {
        // Safari does not allow dst == src canvas.
        // Can't be bothered to work around this.
        return;
    }

    context.save();
    context.set_global_composite_operation("lighter").unwrap();
    context.set_filter("blur(6px)");
    context.draw_image_with_html_canvas_element(&canvas, 0.0, 0.0).unwrap();
    context.restore();
	
}

pub fn vignette(canvas: &HtmlCanvasElement, ctx: &CanvasRenderingContext2d) {

    let width = canvas.width() as f64;
    let height = canvas.height() as f64;
    let center_x = width / 2.0;
    let center_y = height / 2.0;
    let max_radius = (center_x.powi(2) + center_y.powi(2)).sqrt();

    // Save current state
    ctx.save();

    // Set blending mode to multiply
    ctx.set_global_composite_operation("multiply").unwrap();

    // Create radial gradient
    let gradient = ctx.create_radial_gradient(center_x, center_y, 0.0, center_x, center_y, max_radius).unwrap();

    gradient.add_color_stop(0.66, "rgba(255, 255, 255, 1.0)").unwrap(); // White = no change
    gradient.add_color_stop(0.94, "rgba(244, 245, 247, 1.0)").unwrap(); // Black = fully dark

    ctx.set_fill_style_canvas_gradient(&gradient);
    ctx.fill_rect(0.0, 0.0, width, height);

    // Restore previous state
    ctx.restore();
}

fn is_safari() -> bool {
    let user_agent = window().navigator().user_agent().unwrap_or_default();

    // Safari has "Safari" in its UA string, but not "Chrome" or "Chromium"
    user_agent.contains("Safari") && !user_agent.contains("Chrome") && !user_agent.contains("Chromium")
}
