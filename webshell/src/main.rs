use gamecore::*;

use js_sys::Uint8Array;
use log::info;
use num_traits::AsPrimitive as _;
use wasm_bindgen::{JsCast, JsValue, prelude::Closure};
use wasm_bindgen_futures::JsFuture;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, HtmlElement, HtmlImageElement, ImageBitmap, Request, RequestInit, Response, Window};

use std::cell::RefCell;
use std::rc::Rc;

type JsResult<T> = Result<T, JsValue>;

mod http_get;
mod load_image;

use http_get::*;
use load_image::*;

fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Info).expect("error initializing logger");
    wasm_bindgen_futures::spawn_local(async { start().await.expect("main") })
}

struct Res {
    img: ImageBitmap,
}

async fn start() -> JsResult<()> {
    info!("start");
    test_resource_loading().await;

    let img = load_image("kit3.png").await.expect("load img");
    let res = Res { img };
    let mut state = State::new();

    let mut out = Output::new();

    animation_loop(move |ctx| {
        state.tick();

        out.clear();
        state.render(&mut out);

        get_element_by_id::<HtmlElement>("debug").set_inner_text(&out.debug);

        draw(&ctx, &res, &state);
    });

    Ok(())
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

fn draw(ctx: &CanvasRenderingContext2d, res: &Res, state: &State) {
    ctx.set_image_smoothing_enabled(false); // crisp, pixellated sprites
    ctx.draw_image_with_image_bitmap(&res.img, state.x, 0.0).expect("draw");
}

async fn test_resource_loading() {
    info!("say_hello");

    let txt = http_get_with_trunk_hack("test.txt").await.expect("get test.txt");
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
