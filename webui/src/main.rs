use wasm_bindgen::JsValue;
use web_sys::window;

type JsResult<T> = Result<T, JsValue>;

fn main() {
    console_error_panic_hook::set_once();
    wasm_bindgen_futures::spawn_local(async { start().await.expect("main") })
}

async fn start() -> JsResult<()> {
    let document = window()
        .and_then(|win| win.document())
        .expect("Could not access the document");
    let body = document.body().expect("Could not access document.body");
    let text_node = document.create_text_node("Hello, world from Vanilla Rust!");
    body.append_child(text_node.as_ref())
        .expect("Failed to append text");

    Ok(())
}
