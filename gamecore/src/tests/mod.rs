//! Testing approach.
//!
//! Ui interaction (clicking menus etc) is assumed to work -- any breakage will be immediately obvious.
//!
use crate::prelude::*;
use googletest::prelude::*;

mod test_inputs;
mod test_setup;
use test_inputs::*;
use test_setup::*;

// Click a pawn to select it, if in Pointer mode.
#[gtest]
fn pointer_mode_click_selects_pawn() {
    let mut g = small_world();

    let pos = vec2(12, 13);
    let crab1 = g.spawn(Pawn::new(PawnTyp::Crablet, pos));
    let _crab2 = g.spawn(Pawn::new(PawnTyp::Crablet, pos + 3));

    left_click_tile(&mut g, pos);
    tick(&mut g);
    expect_eq!(g.selected_pawn_ids().sorted().collect_vec(), vec![crab1]);
}
