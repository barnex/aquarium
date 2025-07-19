use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{console, window, CanvasRenderingContext2d, HtmlCanvasElement, HtmlImageElement, ImageBitmap};

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
    
    //ctx.fill_rect(4.0, 5.0, 20.0, 30.0);
    
    let img = load_bitmap("kit3.png").await.expect("load img");
    
    ctx.draw_image_with_image_bitmap(&img, 0.0, 0.0).expect("draw");

    Ok(())
}

fn say_hello() {
    let document = window().and_then(|win| win.document()).expect("Could not access the document");
    let body = document.body().expect("Could not access document.body");
    let text_node = document.create_text_node("Hello, world from Vanilla Rust!");
    body.append_child(text_node.as_ref()).expect("Failed to append text");
}


pub async fn load_bitmap(path: &str) -> JsResult<ImageBitmap> {
    console::log_1(&format!("load {path}").into());

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

#[track_caller]
fn get_element<T: JsCast>(id: &str) -> T {
    window().unwrap().document().unwrap().get_element_by_id(id).unwrap().dyn_into::<T>().unwrap()
}
