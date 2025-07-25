mod event_listeners;
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

use engine::*;
use fixed_str::*;
use gamecore::*;
use vector::*;

use js_sys::Uint8Array;
use js_sys::Uint8ClampedArray;
use num_traits::AsPrimitive as _;
use serde::{Serialize, de::DeserializeOwned};
use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsCast, JsValue, prelude::Closure};
use wasm_bindgen_futures::JsFuture;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, HtmlElement, HtmlImageElement, ImageBitmap, ImageData, Request, RequestInit, Response, Window};
use itertools::Itertools as _;

use std::cell::{Cell, RefCell};
use std::collections::VecDeque;
use std::rc::Rc;
use std::str::FromStr as _;
use std::sync::Mutex;

type JsResult<T> = Result<T, JsValue>;
type HashMap<K, V> = fnv::FnvHashMap<K, V>;
type HashSet<T> = fnv::FnvHashSet<T>;
type Shared<T> = Rc<RefCell<T>>;

fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Info).expect("error initializing logger");
    wasm_bindgen_futures::spawn_local(async { start().await.expect("main") })
}

async fn start() -> JsResult<()> {
    log::info!("start");
    test_resource_loading().await;

    let mut res = Res::new(fallback_bitmap(0, 0, 255).await.unwrap());
    let mut state = match load_game() {
        Some(state) => {
            log::info!("game loaded");
            state
        }
        None => {
            log::error!("game not loaded, starting fresh");
            State::new()
        }
    };

    let mut out = Output::new();
    let canvas = get_element_by_id("canvas");

    // queue where we receive input events (keys, mouse)
    let input_events = Shared::<VecDeque<InputEvent>>::default();
    listen_keys(Rc::clone(&input_events));
    listen_mouse(&canvas, Rc::clone(&input_events));

    animation_loop(move |ctx| {
        state.inputs.now_secs = now_secs();
        record_input_events(&mut state.inputs, &input_events);

        state.tick();

        out.clear();
        state.render(&mut out);
        ctx.clear_rect(0.0, 0.0, canvas.width().as_(), canvas.height().as_());
        res.poll();
        draw(&ctx, &mut res, &out);

        get_element_by_id::<HtmlElement>("debug").set_inner_text(&out.debug);

        exec_commands(&mut state);

        if state.request_save {
            save_game(&mut state);
            state.request_save = false;
        }
    });

    Ok(())
}

//fn autosave_handler() {
//    let closure = Closure::wrap(Box::new(move |event: Event| {
//        log::info!("Window is about to close!");
//    }) as Box<dyn FnMut(_)>);
//    window().add_event_listener_with_callback("beforeunload", closure.as_ref().unchecked_ref()).unwrap();
//    closure.forget();
//}
//

const APP_KEY: &str = "a_strategy_game_data_v01";

fn save_game(state: &State) {
    log::info!("autosave... {APP_KEY}");
    serialize(APP_KEY, state).expect("autosave");
}

fn load_game() -> Option<State> {
    log::info!("loading... {APP_KEY}");
    deserialize(APP_KEY).map_err(|e| log::error!("load_game {APP_KEY}: {e:?}")).ok()
}

#[wasm_bindgen]
pub async fn fallback_bitmap(r: u8, g: u8, b: u8) -> Result<ImageBitmap, JsValue> {
    let width = 32;
    let height = 32;
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
fn record_input_events(inputs: &mut Inputs, events: &Shared<VecDeque<InputEvent>>) {
    inputs.start_next_frame();

    for event in events.borrow_mut().drain(..) {
        use InputEvent::*;
        match event {
            KeyDown(event) => {
                if let Ok(key) = Str16::from_str(&event.key()) {
                    inputs.record_press(Button(key))
                }
            }
            KeyUp(event) => {
                if let Ok(key) = Str16::from_str(&event.key()) {
                    inputs.record_release(Button(key))
                }
            }
            MouseDown(event) => {
                match event.button() {
                    0 => inputs.record_press(Button::MOUSE1),
                    2 => inputs.record_press(Button::MOUSE2),
                    _ => (),
                }
                // ⚠️ use `offset_x` for relative position inside canvas
                inputs.record_mouse_position(vec2(event.offset_x().as_(), event.offset_y().as_()));
            }
            MouseUp(event) => {
                match event.button() {
                    0 => inputs.record_release(Button::MOUSE1),
                    2 => inputs.record_release(Button::MOUSE2),
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

fn draw(ctx: &CanvasRenderingContext2d, res: &mut Res, out: &Output) {
    ctx.set_image_smoothing_enabled(false); // crisp, pixellated sprites

    for (sprite, pos) in &out.sprites {
        if let Some(bitmap) = res.get(sprite) {
            ctx.draw_image_with_image_bitmap(bitmap, pos.x().as_(), pos.y().as_()).expect("draw");
        }
    }
}

async fn test_resource_loading() {
    log::info!("say_hello");

    let txt = http_get_with_trunk_hack("assets/test.txt").await.expect("get test.txt");
    let txt = String::from_utf8_lossy(&txt);

    let document = window().document().unwrap();
    let body = document.body().unwrap();
    let text_node = document.create_text_node(&txt);
    body.append_child(text_node.as_ref()).unwrap();
}

pub fn window() -> Window {
    web_sys::window().expect("window")
}

#[track_caller]
fn get_element_by_id<T: JsCast>(id: &str) -> T {
    web_sys::window().unwrap().document().unwrap().get_element_by_id(id).unwrap().dyn_into::<T>().unwrap()
}
