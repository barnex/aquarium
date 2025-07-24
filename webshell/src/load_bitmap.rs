use crate::*;

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
