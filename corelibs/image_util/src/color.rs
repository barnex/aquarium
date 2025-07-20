use anyhow::{bail, Result};
use vector::*;

#[allow(unused)]
pub const RED: vec3f = vec3f(1.0, 0.0, 0.0);
pub const GREEN: vec3f = vec3f(0.0, 1.0, 0.0);
pub const BLUE: vec3f = vec3f(0.0, 0.0, 1.0);
pub const WHITE: vec3f = vec3f(1.0, 1.0, 1.0);
pub const BLACK: vec3f = vec3f(0.0, 0.0, 0.0);

pub fn parse_hex_color(color: &str) -> Result<vec3u8> {
	let color = color.strip_prefix('#').unwrap_or(color);
	if color.len() != 6 {
		bail!("parse hex colour `{color}`: need 6 characters")
	}
	let r = u8::from_str_radix(&color[0..2], 16)?;
	let g = u8::from_str_radix(&color[2..4], 16)?;
	let b = u8::from_str_radix(&color[4..6], 16)?;
	Ok(vec3(r, g, b))
}
