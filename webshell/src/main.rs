mod event_listeners;
mod graphics_postprocessing;
mod http_get;
mod js_commands;
mod load_bitmap;
mod resources;
mod storage;
mod time;

use event_listeners::*;
use http_get::*;
use js_commands::*;
use load_bitmap::*;
use resources::*;
use storage::*;
use time::*;

use fixed_str::*;
use gamecore::*;
use vector::*;

pub use itertools::Itertools as _;
use js_sys::Uint8Array;
use js_sys::Uint8ClampedArray;
use num_traits::AsPrimitive as _;
use serde::{Serialize, de::DeserializeOwned};
use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsCast, JsValue, prelude::Closure};
use wasm_bindgen_futures::JsFuture;
use web_sys::HtmlAnchorElement;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, HtmlElement, HtmlImageElement, ImageBitmap, ImageData, KeyboardEvent, MouseEvent, Request, RequestInit, Response, Window};

use std::cell::{Cell, RefCell};
use std::collections::VecDeque;
use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;
use std::str::FromStr as _;
use std::sync::Mutex;

type JsResult<T> = Result<T, JsValue>;
type HashMap<K, V> = fnv::FnvHashMap<K, V>;
type HashSet<T> = fnv::FnvHashSet<T>;
type Shared<T> = Rc<RefCell<T>>;

fn main() {
    web_sys::console::log_1(&"WASM main started. Hello from Rust.".into());
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Info).expect("error initializing logger");
    wasm_bindgen_futures::spawn_local(async { start().await.expect("main") })
}

async fn start() -> JsResult<()> {
    log::info!("async fn start spawned. Hello from async Rust.");
    test_resource_loading().await;

    let mut res = Resources::new(fallback_bitmap((0, 0, 255), TILE_SIZE).await.unwrap());
    let mut state = match load_game() {
        Some(state) => {
            log::info!("game loaded");
            state
        }
        None => {
            log::error!("game not loaded, starting fresh");
            G::new()
        }
    };

    //let mut out = Output::new();
    let canvas = get_element_by_id("canvas");

    // queue where we receive input events (keys, mouse)
    let input_events = Shared::<VecDeque<InputEvent>>::default();
    listen_keys(Rc::clone(&input_events));
    listen_mouse(&canvas, Rc::clone(&input_events));

    let mut out = Out::default();

    // 🌍 Main loop
    animation_loop(move |ctx| {
        state.now_secs = now_secs();
        state.viewport_size = vec2(canvas.width(), canvas.height());
        record_input_events(&state.keymap, &mut state.inputs, &input_events);

        out.clear();
        state.tick(&mut out);

        //out.clear();
        //state.render();

        ctx.clear_rect(0.0, 0.0, canvas.width().as_(), canvas.height().as_());
        res.poll();

        draw(&canvas, &ctx, &mut res, &out);

        get_element_by_id::<HtmlElement>("debug").set_inner_text(&out.debug);

        exec_pending_commands(&mut state);
    });

    Ok(())
}

const APP_KEY: &str = "a_strategy_game_data_v01";

fn save_game(state: &G) {
    log::info!("autosave... {APP_KEY}");
    serialize(APP_KEY, state).expect("autosave");
}

fn load_game() -> Option<G> {
    log::info!("loading... {APP_KEY}");
    deserialize(APP_KEY).map_err(|e| log::error!("load_game {APP_KEY}: {e:?}")).ok()
}

pub async fn fallback_bitmap((r, g, b): (u8, u8, u8), size: u32) -> Result<ImageBitmap, JsValue> {
    let width = size as usize;
    let height = size as usize;
    let num_pixels = width * height;
    let mut rgba = Vec::with_capacity(num_pixels * 4);

    // Fill with solid red (R=255, G=0, B=0, A=255)
    for _ in 0..num_pixels {
        rgba.push(r);
        rgba.push(g);
        rgba.push(b);
        rgba.push(255); // A
    }

    // Convert Vec<u8> → Uint8ClampedArray
    let clamped = Uint8ClampedArray::new_with_length((num_pixels * 4) as u32);
    clamped.copy_from(&rgba[..]);

    // Wrap in ImageData
    let image_data = ImageData::new_with_js_u8_clamped_array_and_sh(&clamped, width as u32, height as u32)?;

    // Create ImageBitmap from ImageData
    let promise = window().create_image_bitmap_with_image_data(&image_data)?;
    let js_value = JsFuture::from(promise).await?;
    let bitmap = js_value.dyn_into::<ImageBitmap>()?;

    Ok(bitmap)
}

fn animation_loop<F>(mut body: F)
where
    F: FnMut(&CanvasRenderingContext2d) + 'static,
{
    let canvas = get_element_by_id::<HtmlCanvasElement>("canvas");

    let ctx = canvas.get_context("2d").expect("context2d").unwrap().dyn_into::<CanvasRenderingContext2d>().unwrap();

    let anim_loop: Rc<RefCell<Option<Closure<dyn FnMut()>>>> = Rc::new(RefCell::new(None));
    let anim_loop_clone = anim_loop.clone();

    *anim_loop_clone.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        ctx.clear_rect(0.0, 0.0, canvas.width().as_(), canvas.height().as_());

        body(&ctx);

        request_animation_frame(&anim_loop);
    }) as Box<dyn FnMut()>));

    // Start animation loop
    request_animation_frame(&anim_loop_clone);
}

// take input events from queue and update Inputs state accordingly
fn record_input_events(keymap: &Keymap, inputs: &mut Inputs, events: &Shared<VecDeque<InputEvent>>) {
    inputs.start_next_frame();

    for event in events.borrow_mut().drain(..) {
        use InputEvent::*;
        match event {
            KeyDown(event) => {
                if let Ok(key) = Str16::from_str(&event.key()) {
                    inputs.record_press(keymap, Button(key))
                }
            }
            KeyUp(event) => {
                if let Ok(key) = Str16::from_str(&event.key()) {
                    inputs.record_release(keymap, Button(key))
                }
            }
            MouseDown(event) => {
                match event.button() {
                    0 => inputs.record_press(keymap, K_MOUSE1),
                    2 => inputs.record_press(keymap, K_MOUSE2),
                    _ => (),
                }
                // ⚠️ use `offset_x` for relative position inside canvas
                inputs.record_mouse_position(vec2(event.offset_x().as_(), event.offset_y().as_()));
            }
            MouseUp(event) => {
                match event.button() {
                    0 => inputs.record_release(keymap, K_MOUSE1),
                    2 => inputs.record_release(keymap, K_MOUSE2),
                    _ => (),
                }

                // ⚠️ use `offset_x` for relative position inside canvas
                inputs.record_mouse_position(vec2(event.offset_x().as_(), event.offset_y().as_()));
            }
            MouseMove(event) => {
                // ⚠️ use `offset_x` for relative position inside canvas
                inputs.record_mouse_position(vec2(event.offset_x().as_(), event.offset_y().as_()));
            }
        }
    }
}

fn request_animation_frame(anim_loop_clone: &Rc<RefCell<Option<Closure<dyn FnMut()>>>>) -> i32 {
    window().request_animation_frame(anim_loop_clone.borrow().as_ref().unwrap().as_ref().unchecked_ref()).unwrap()
}

fn draw(canvas: &HtmlCanvasElement, ctx: &CanvasRenderingContext2d, res: &mut Resources, out: &Out) {
    ctx.set_image_smoothing_enabled(false); // crisp, pixellated sprites

    // Draw layers starting from 0 for correct Z-ordering.
    for Layer { sprites, lines, rectangles } in &out.layers {
        // ▭ rectangles
        for rect in rectangles {
            if rect.fill != RGBA::TRANSPARENT {
                ctx.set_fill_style_str(&rect.fill.hex());
                ctx.fill_rect(rect.bounds.min.x().as_(), rect.bounds.min.y().as_(), rect.bounds.size().x().as_(), rect.bounds.size().y().as_());
            }

            if rect.stroke != RGBA::TRANSPARENT {
                ctx.set_stroke_style_str(&rect.stroke.hex());
                ctx.set_line_width(1.0);
                // 👇 HTML Canvas aligns strokes (but not fills) to the edges of pixels instead of to the center.
                // Offset by half a pixel to align pixel-perfect.
                ctx.stroke_rect((rect.bounds.min.x() as f64) + 0.5, (rect.bounds.min.y() as f64) + 0.5, (rect.bounds.size().x() as f64) - 1.0, (rect.bounds.size().y() as f64) - 1.0);
            }
        }

        // 🦀 sprites
        for cmd in sprites {
            if let Some(bitmap) = res.get(&cmd.sprite) {
                ctx.draw_image_with_image_bitmap_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                    bitmap,
                    0.0,                   // source x
                    0.0,                   // source y
                    bitmap.width().as_(),  // source width
                    bitmap.height().as_(), // source height
                    cmd.pos.x().as_(),         // dest x
                    cmd.pos.y().as_(),         // dest y
                    bitmap.width().as_(),  // dest width
                    bitmap.height().as_(), // dest height
                )
                .expect("draw");
            }
        }

        // ╱ lines
        for line in lines {
            ctx.begin_path();
            ctx.set_stroke_style_str(&line.color.hex());
            ctx.set_line_width(line.width.as_());
            ctx.move_to(line.start.x().as_(), line.start.y().as_());
            ctx.line_to(line.end.x().as_(), line.end.y().as_());
            ctx.stroke();
        }
    }

    //graphics_postprocessing::inverse_bloom(canvas, ctx);
    graphics_postprocessing::vignette(canvas, ctx);
}

pub fn window() -> Window {
    web_sys::window().expect("window")
}

#[track_caller]
fn get_element_by_id<T: JsCast>(id: &str) -> T {
    web_sys::window().unwrap().document().unwrap().get_element_by_id(id).unwrap().dyn_into::<T>().unwrap()
}

pub fn download_screenshot(filename: &str) -> JsResult<()> {
    // Get document and canvas
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    let canvas = get_element_by_id::<HtmlCanvasElement>("canvas");

    // Get canvas as data URL (PNG)
    let data_url = canvas.to_data_url()?; // defaults to image/png

    // Create anchor element
    let a = document.create_element("a")?.dyn_into::<HtmlAnchorElement>()?;

    a.set_href(&data_url);
    a.set_download(filename);
    a.click();
    Ok(())
}
