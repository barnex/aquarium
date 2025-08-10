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
fn click_selects_pawn() {
    // given:
    let mut g = small_world();
    g.ui.hidden = true; // don't accidentally click on UI
    g.ui.active_tool = Tool::Pointer;

    // add two pawns
    let pos1 = vec2(12, 13);
    let crab1 = g.spawn(Pawn::new(PawnTyp::Crablet, pos1));

    let pos2 = vec2(14, 15);
    let crab2 = g.spawn(Pawn::new(PawnTyp::Crablet, pos2));

    // click on one
    left_click_tile(&mut g, pos1);
    tick(&mut g);

    // clicked pawn should be selected
    expect_eq!(g.selected_pawn_ids().sorted().collect_vec(), vec![crab1], "clicked pawn should be selected");

    // second click on already selected pawn does nothing
    left_click_tile(&mut g, pos1);
    tick(&mut g);
    expect_eq!(g.selected_pawn_ids().sorted().collect_vec(), vec![crab1], "clicked pawn should be selected");

    // click on other pawn
    left_click_tile(&mut g, pos2);
    tick(&mut g);
    expect_eq!(g.selected_pawn_ids().sorted().collect_vec(), vec![crab2], "clicked pawn should be selected");
}

#[gtest]
fn drag_selects_pawn() {
    let mut g = small_world();
    g.ui.hidden = true; // don't accidentally click on UI
    g.ui.active_tool = Tool::Pointer;

    // add two pawns
    let pos1 = vec2(10, 11);
    let crab1 = g.spawn(Pawn::new(PawnTyp::Crablet, pos1));

    let pos2 = vec2(13, 14);
    let crab2 = g.spawn(Pawn::new(PawnTyp::Crablet, pos2));

    left_mousedown_tile(&mut g, vec2(9, 8));
    tick(&mut g);

    mousemove_tile(&mut g, vec2(15, 16));
    tick(&mut g);

    left_mouseup_tile(&mut g, vec2(15, 16));
    tick(&mut g);

    expect_eq!(g.selected_pawn_ids().sorted().collect_vec(), vec![crab1, crab2], "drag to select");
}
