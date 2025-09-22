use std::str::FromStr;

use proc_macro2::TokenStream;
use proc_macros_impl::*;

fn main() {
	let code = r#"
pub struct Crablet {
	// tile position on the map
	pub position: Mut<vec2i>,

	pub home_area: vec2i, // << RM

	// keeps previous tile congested until fully moved away
	pub prev_pos: vec2i,

	// animated position smoothly trails behind tile position
	pub anim_pos: vec2f,
	pub anim_timer: Timer,

	// movement state
	pub navigation: NavState,

	// work state
	pub task: Task,
	pub cargo: Option<Resource>,

	pub selected: bool,

	pub scale: f32,
	pub rotation: f32,
	pub sprite: Sprite,
}
	"#;

	let input = TokenStream::from_str(code).expect("tokenize");

	let output = derive_setters(input);

	println!("{output}");
}
