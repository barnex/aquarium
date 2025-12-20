use crate::*;
use futures_util::StreamExt;

// Load image over HTTP. E.g. `assets/my_sprite.png`

pub async fn load_bitmap(url: &str) -> Result<ImageBitmap, JsValue> {
    log::info!("load_bitmap {url}");
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    let img = document.create_element("img")?.dyn_into::<HtmlImageElement>()?;
    img.set_src(url);

    // Use mpsc channel to handle load/error
    let (mut tx, mut rx) = futures::channel::mpsc::channel::<Result<(), JsValue>>(1);

    // Load handler
    let mut success_tx = tx.clone();
    let onload = Closure::<dyn FnMut()>::wrap(Box::new(move || {
        let _ = success_tx.try_send(Ok(()));
    }));

    // Error handler
    let onerror = Closure::<dyn FnMut()>::wrap(Box::new(move || {
        let _ = tx.try_send(Err(JsValue::from_str("Image failed to load")));
    }));

    img.set_onload(Some(onload.as_ref().unchecked_ref()));
    img.set_onerror(Some(onerror.as_ref().unchecked_ref()));
    onload.forget();
    onerror.forget();

    // Wait for the first event (load or error)
    match rx.next().await {
        Some(Ok(())) => {
            if img.natural_width() == 0 || img.natural_height() == 0 {
                return Err(JsValue::from_str("Image dimensions are zero — invalid image"));
            }

            let promise = window.create_image_bitmap_with_html_image_element(&img).map_err(|e| JsValue::from(format!("ImageBitmap creation failed: {:?}", e)))?;

            let bitmap = JsFuture::from(promise).await?;
            Ok(bitmap.dyn_into::<ImageBitmap>()?)
        }
        Some(Err(e)) => Err(e),
        None => Err(JsValue::from_str("Image load channel closed unexpectedly")),
    }
}
