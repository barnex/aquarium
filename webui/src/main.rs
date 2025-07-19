use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;
use web_sys::{HtmlCanvasElement, window};

type JsResult<T> = Result<T, JsValue>;

fn main() {
    console_error_panic_hook::set_once();
    wasm_bindgen_futures::spawn_local(async { start().await.expect("main") })
}

async fn start() -> JsResult<()> {
    let document = window().unwrap().document().unwrap();
    say_hello();

    let canvas = get_element::<HtmlCanvasElement>("canvas");

    let ctx = canvas.get_context("2d").expect("context2d").unwrap().dyn_into::<CanvasRenderingContext2d>().unwrap();
    
    ctx.fill_rect(4.0, 5.0, 20.0, 30.0);

    Ok(())
}

fn say_hello() {
    let document = window().and_then(|win| win.document()).expect("Could not access the document");
    let body = document.body().expect("Could not access document.body");
    let text_node = document.create_text_node("Hello, world from Vanilla Rust!");
    body.append_child(text_node.as_ref()).expect("Failed to append text");
}

#[track_caller]
fn get_element<T: JsCast>(id: &str) -> T {
    window().unwrap().document().unwrap().get_element_by_id(id).unwrap().dyn_into::<T>().unwrap()
}
