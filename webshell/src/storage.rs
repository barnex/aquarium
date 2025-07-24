use crate::*;
use base64::Engine as _;

pub fn serialize<T>(key: &str, v: &T) -> JsResult<()>
where
    T: Serialize,
{
    let bytes = bincode::serde::encode_to_vec(v, bincode::config::standard()).map_err(|e| format!("encode {key:?}: {e:?}"))?;
    save_bytes(key, &bytes)
}

pub fn deserialize<T>(key: &str) -> JsResult<T> where T:DeserializeOwned{
	let bytes = load_bytes(key)?;
	let (v, _) = bincode::serde::decode_from_slice(&bytes, bincode::config::standard()).map_err(|e| format!("decode {key:?}: {e:?}"))?;
	Ok(v)
}

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
