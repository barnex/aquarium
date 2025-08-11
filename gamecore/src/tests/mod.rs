//! Testing approach.
//!
//! Ui interaction (clicking menus etc) is assumed to work -- any breakage will be immediately obvious.
//!
use crate::prelude::*;
use googletest::prelude::*;

mod test_inputs;
mod test_setup;
mod headless_renderer;
use test_inputs::*;
use test_setup::*;
use headless_renderer::*;

// Click a pawn to select it, if in Pointer mode.
#[gtest]
fn click_selects_pawn() {
    // given:
    let g = &mut small_world();
    g.ui.active_tool = Tool::Pointer;

    // add two pawns
    let pos1 = vec2(12, 13);
    let crab1 = g.spawn(Pawn::new(PawnTyp::Crablet, pos1));

    let pos2 = vec2(14, 15);
    let crab2 = g.spawn(Pawn::new(PawnTyp::Crablet, pos2));

    tick(g, click_tile(pos1));
    expect_eq!(g.selected_pawn_ids().sorted().collect_vec(), vec![crab1].sorted(), "clicked pawn should be selected");

    // second click on already selected pawn does nothing
    tick(g, click_tile(pos1));
    expect_eq!(g.selected_pawn_ids().sorted().collect_vec(), vec![crab1].sorted(), "clicked pawn should be selected");

    // click on other pawn
    tick(g, click_tile(pos2));
    expect_eq!(g.selected_pawn_ids().sorted().collect_vec(), vec![crab2].sorted(), "clicked pawn should be selected");

    screenshot(g);

    // deselect by clicking elsewhere
    tick(g, click_tile(pos2 + 12));
    expect_eq!(g.selected_pawn_ids().sorted().collect_vec(), vec![].sorted(), "should de-select");
}

#[gtest]
/// Select pawns by dragging a rectangle.
fn drag_selects_pawn() {
    let g = &mut small_world();
    g.ui.active_tool = Tool::Pointer;

    // add two pawns
    let pos1 = vec2(10, 11);
    let crab1 = g.spawn(Pawn::new(PawnTyp::Crablet, pos1));

    let pos2 = vec2(13, 14);
    let crab2 = g.spawn(Pawn::new(PawnTyp::Crablet, pos2));

    tick(g, [mouse_move_tile((9, 9))]);
    tick(g, [mouse_down()]);
    tick(g, [mouse_move_tile((15, 16)), mouse_up()]);

    expect_eq!(g.selected_pawn_ids().sorted().collect_vec(), vec![crab1, crab2].sorted(), "drag to select");
}
