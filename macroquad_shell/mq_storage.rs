use crate::*;
use anyhow::Context as _;
use anyhow::Result;
use core_util::*;
use serde::{Serialize, de::DeserializeOwned};
use std::fs;

const APP_KEY: &'static str = "savegame";

pub fn save_game(state: &G) {
    log::info!("save {APP_KEY}");
    serialize(APP_KEY, state).expect("autosave");
}

pub fn load_game() -> Option<G> {
    log::info!("loading... {APP_KEY}");
    deserialize(APP_KEY).map_err(|e| log::info!("load_game {APP_KEY}: {e}")).ok()
}

/// Serialize value to browser storage under given key.
pub fn serialize<T>(key: &str, v: &T) -> Result<()>
where
    T: Serialize,
{
    let bytes = bincode::serde::encode_to_vec(v, bincode::config::standard()).with_context(|| format!("encode {key:?}"))?;
    save_bytes(key, &bytes)
}

/// Deserialize from browser storage under given key.
pub fn deserialize<T>(key: &str) -> Result<T>
where
    T: DeserializeOwned,
{
    let bytes = load_bytes(key)?;
    let (v, _) = bincode::serde::decode_from_slice(&bytes, bincode::config::standard()).with_context(|| format!("decode {key:?}"))?;
    Ok(v)
}

/// Store data to browser storage under given key.
pub fn save_bytes(key: &str, data: &[u8]) -> Result<()> {
    let path = format!("app_storage/{key}");
    log::info!("save {path:?}");
    Ok(fs::write(path, data)?)
}

/// Load data from browser storage under given key.
pub fn load_bytes(key: &str) -> Result<Vec<u8>> {
    let path = format!("app_storage/{key}");
    log::info!("load {path:?}");
    Ok(fs::read(path)?)
}
