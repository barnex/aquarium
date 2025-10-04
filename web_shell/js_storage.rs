//! Persist data in browser storage.
use crate::*;
use base64::Engine as _;

pub fn save_game<G:GameCore>(state: &G) {
    log::info!("save... {APP_KEY}");
    serialize(APP_KEY, state).expect("autosave");
}

pub fn load_game<G:GameCore>() -> Option<G> {
    log::info!("loading... {APP_KEY}");
    deserialize(APP_KEY).map_err(|e| log::error!("load_game {APP_KEY}: {e:?}")).ok()
}

/// Serialize value to browser storage under given key.
pub fn serialize<T>(key: &str, v: &T) -> JsResult<()>
where
    T: Serialize,
{
    let bytes = bincode::serde::encode_to_vec(v, bincode::config::standard()).map_err(|e| format!("encode {key:?}: {e:?}"))?;
    save_bytes(key, &bytes)
}

/// Deserialize from browser storage under given key.
pub fn deserialize<T>(key: &str) -> JsResult<T>
where
    T: DeserializeOwned,
{
    let bytes = load_bytes(key)?;
    let (v, _) = bincode::serde::decode_from_slice(&bytes, bincode::config::standard()).map_err(|e| format!("decode {key:?}: {e:?}"))?;
    Ok(v)
}

/// Store data to browser storage under given key.
pub fn save_bytes(key: &str, data: &[u8]) -> JsResult<()> {
    let storage = window().local_storage().unwrap().unwrap();
    let encoded = base64::engine::general_purpose::STANDARD.encode(data);
    storage.set_item(key, &encoded)
}

/// Load data from browser storage under given key.
pub fn load_bytes(key: &str) -> JsResult<Vec<u8>> {
    let storage = window().local_storage().unwrap().unwrap();
    let s = storage.get_item(key)?.unwrap_or_else(|| "load_bytes {key:?}: not found".into());
    base64::engine::general_purpose::STANDARD.decode(&s).map_err(|e| format!("decode bytes for {key:?}: {e}").into())
}
