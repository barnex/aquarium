use crate::*;
use base64::Engine as _;

pub fn save_bytes(key: &str, data: &[u8]) -> JsResult<()> {
    let storage = window().local_storage().unwrap().unwrap();
    let encoded = base64::engine::general_purpose::STANDARD.encode(data);
    storage.set_item(key, &encoded)
}

pub fn load_bytes(key: &str) -> JsResult<Vec<u8>> {
    let storage = window().local_storage().unwrap().unwrap();
    let s = storage.get_item(key)?.unwrap_or_else(|| "load_bytes {key:?}: not found".into());
    base64::engine::general_purpose::STANDARD.decode(&s).map_err(|e| format!("decode bytes for {key:?}: {e}").into())
}
