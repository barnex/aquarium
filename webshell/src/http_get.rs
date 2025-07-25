//! Fetch data over HTTP.
use crate::*;

// Trunk annoyingly returns index.html when a file is not found.
// Work around this by recognizing a magic comment in index.html and returning an error instead.
pub async fn http_get_with_trunk_hack(url: &str) -> JsResult<Vec<u8>> {
    let bytes = http_get(url).await?;
    if bytes.starts_with(b"<!DOCTYPE html> <!-- This comment identifies index.html DO NOT REMOVE -->") {
        return Err(format!("GET {url}: got some HTML, presumably Trunk 404").into());
    }
    Ok(bytes)
}

// WARNING: ⚠️ Trunk serves index.html instead of 404 Not Found.
// See `http_get_with_trunk_hack`.
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
