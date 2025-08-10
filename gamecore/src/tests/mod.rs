//! Testing approach.
//! 
//! Ui interaction (clicking menus etc) is assumed to work -- any breakage will be immediately obvious.
//! 
use std::sync::OnceLock;

use crate::prelude::*;
use googletest::prelude::*;

mod test_setup;
mod test_inputs;
use test_setup::*;
use test_inputs::*;


// Click a pawn to select it, if in Pointer mode.
#[gtest]
fn pointer_mode_click_selects_pawn(){
	let mut g = small_world();

	let pos = vec2(12, 13);
	let crab1 = g.spawn(Pawn::new(PawnTyp::Crablet, pos));

	left_click_tile(&mut g, pos);
	g.tick(&mut default());
	assert_that!(g.frame, eq(1));

	//verify_that!(g.selected_pawn_ids().collect_vec(), unordered_elements_are!([eq(&crab1)]));
	expect_eq!(g.selected_pawn_ids().sorted().collect_vec(), vec![crab1]);
}
