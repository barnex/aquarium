use std::{cell::RefCell, rc::Rc};

use js_sys::Uint8Array;
use log::info;
use wasm_bindgen::{JsCast, JsValue, prelude::Closure};
use wasm_bindgen_futures::JsFuture;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, HtmlImageElement, ImageBitmap, Request, RequestInit, Response, Window, console};
use console_log;

type JsResult<T> = Result<T, JsValue>;

fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Info).expect("error initializing logger");
    wasm_bindgen_futures::spawn_local(async { start().await.expect("main") })
}

struct State {
    x: f64,
}

impl State {
    fn tick(&mut self) {
        self.x += 1.0;
        if self.x > 100.0 {
            self.x = 0.0
        }
    }
}

struct Res {
    img: ImageBitmap,
}


async fn start() -> JsResult<()> {
    info!("start");

    let document = window().document().unwrap();
    say_hello().await;

    let canvas = get_element::<HtmlCanvasElement>("canvas");

    let ctx = canvas.get_context("2d").expect("context2d").unwrap().dyn_into::<CanvasRenderingContext2d>().unwrap();

    //ctx.fill_rect(4.0, 5.0, 20.0, 30.0);

    let img = load_bitmap("kit3.png").await.expect("load img");
    let res = Res { img };

    let anim_loop: Rc<RefCell<Option<Closure<dyn FnMut()>>>> = Rc::new(RefCell::new(None));
    let anim_loop_clone = anim_loop.clone();

    let mut state = State { x: 0.0 };

    *anim_loop_clone.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        ctx.clear_rect(0.0, 0.0, 100.0, 100.0);
        state.tick();
        draw(&ctx, &res, &state);

        window().request_animation_frame(anim_loop.borrow().as_ref().unwrap().as_ref().unchecked_ref()).unwrap();
    }) as Box<dyn FnMut()>));

    // Start animation loop
    window().request_animation_frame(anim_loop_clone.borrow().as_ref().unwrap().as_ref().unchecked_ref()).unwrap();

    Ok(())
}

fn draw(ctx: &CanvasRenderingContext2d, res: &Res, state: &State) {
    ctx.draw_image_with_image_bitmap(&res.img, state.x, 0.0).expect("draw");
}

async fn say_hello() {
    info!("say_hello");

    let txt = http_get_with_trunk_hack("test.txt").await.expect("get test.txt");
    let txt = String::from_utf8_lossy(&txt);

    let document = window().document().unwrap();
    let body = document.body().unwrap();
    let text_node = document.create_text_node(&txt);
    body.append_child(text_node.as_ref()).unwrap();
}

pub async fn load_bitmap(path: &str) -> JsResult<ImageBitmap> {
    info!("load {path}");

    // Load image element
    let img = HtmlImageElement::new()?;

    // Wait for image to load
    let (load_tx, load_rx) = futures_channel::oneshot::channel();
    let onload = Closure::once(Box::new(move || {
        let _ = load_tx.send(());
    }) as Box<dyn FnOnce()>);
    img.set_onload(Some(onload.as_ref().unchecked_ref()));
    onload.forget();
    img.set_src(path);
    load_rx.await.map_err(|_| JsValue::from_str("image load failed"))?;

    // Create ImageBitmap from image element
    let bitmap_promise = web_sys::window().unwrap().create_image_bitmap_with_html_image_element(&img)?;
    let bitmap_jsvalue = JsFuture::from(bitmap_promise).await?;
    let bitmap: ImageBitmap = bitmap_jsvalue.dyn_into()?;
    Ok(bitmap)
}

// Trunk annoyingly returns index.html when a file is not found.
// Work around this by recognizing a magic comment in index.html and returning an error instead.
async fn http_get_with_trunk_hack(url: &str) -> JsResult<Vec<u8>> {
    let bytes = http_get(url).await?;
    if bytes.starts_with(b"<!DOCTYPE html> <!-- This comment identifies index.html DO NOT REMOVE -->") {
        return Err(format!("GET {url}: got some HTML, presumably Trunk 404").into());
    }
    Ok(bytes)
}

async fn http_get(url: &str) -> JsResult<Vec<u8>> {
    let opts = RequestInit::new();
    opts.set_method("GET");
    opts.set_mode(web_sys::RequestMode::Cors);
    let request = Request::new_with_str_and_init(url, &opts)?;

    let response = JsFuture::from(window().fetch_with_request(&request)).await?.dyn_into::<Response>()?;
    if !response.ok() {
        return Err(JsValue::from_str(&format!("GET {url}: status {}: {}", response.status(), response.status_text())));
    }

    let array_buffer = JsFuture::from(response.array_buffer()?).await?;
    let bytes = Uint8Array::new(&array_buffer).to_vec();

    Ok(bytes)
}

pub fn window() -> Window {
    web_sys::window().expect("window")
}

#[track_caller]
fn get_element<T: JsCast>(id: &str) -> T {
    web_sys::window().unwrap().document().unwrap().get_element_by_id(id).unwrap().dyn_into::<T>().unwrap()
}
