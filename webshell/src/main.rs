mod event_listeners;
mod graphics_postprocessing;
mod http_get;
mod js_commands;
mod js_renderer;
mod js_resources;
mod load_bitmap;
mod js_storage;
mod time;

use event_listeners::*;
use http_get::*;
use js_commands::*;
use js_renderer::*;
use js_resources::*;
use load_bitmap::*;
use shell_api::*;
use js_storage::*;
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
    #[cfg(debug_assertions)]
    {
        log::warn!("debug_assertions enabled, performance will suffer");
    }
    wasm_bindgen_futures::spawn_local(async { start().await.expect("main") })
}

async fn start() -> JsResult<()> {
    log::info!("async fn start spawned. Hello from async Rust.");
    test_resource_loading().await;

    let mut res = Resources::new(fallback_bitmap((0, 0, 255), TILE_SIZE).await.unwrap());
    let mut g = match load_game() {
        Some(state) => {
            log::info!("game loaded");
            state
        }
        None => {
            log::error!("game not loaded, starting fresh");
            let mut g = G::test_world();
            #[cfg(debug_assertions)]
            {
                log::info!("enabling pause_on_sanity_failure (because debug_assertions)");
                g.debug.pause_on_sanity_failure = true;
            }
            g
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
        out.clear();
        out.viewport_size = vec2(canvas.width(), canvas.height());

        g.tick(now_secs(), input_events.borrow_mut().drain(..), &mut out);

        draw(&canvas, &ctx, &mut res, &out);

        get_element_by_id::<HtmlElement>("debug").set_inner_text(&out.debug);

        exec_pending_commands(&mut g);
    });

    Ok(())
}

const APP_KEY: &str = "a_strategy_game_data_v01";


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

fn request_animation_frame(anim_loop_clone: &Rc<RefCell<Option<Closure<dyn FnMut()>>>>) -> i32 {
    window().request_animation_frame(anim_loop_clone.borrow().as_ref().unwrap().as_ref().unchecked_ref()).unwrap()
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
