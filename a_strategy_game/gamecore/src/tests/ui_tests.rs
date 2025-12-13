//! Tests that require UI interaction (clicking etc).
use super::*;
use crate::*;
use googletest::prelude::*;

// Click a pawn to select it, if in Pointer mode.
#[gtest]
fn click_selects_pawn() {
    // given:
    let g = &mut small_world(caller!());
    g.ui.active_tool = Tool::Pointer;

    // add two pawns
    let pos1 = vec2(12, 13);
    let crab1 = g.spawn_pawn(Pawn::new(PawnTyp::Crablet, pos1, g.player)).id;

    let pos2 = vec2(14, 15);
    let crab2 = g.spawn_pawn(Pawn::new(PawnTyp::Crablet, pos2, g.player)).id;

    tick(g, mouse_click_tile(pos1));
    expect_eq!(g.selected_pawn_ids().sorted().collect_vec(), vec![crab1].sorted(), "clicked pawn should be selected");

    // second click on already selected pawn does nothing
    tick(g, mouse_click_tile(pos1));
    expect_eq!(g.selected_pawn_ids().sorted().collect_vec(), vec![crab1].sorted(), "clicked pawn should be selected");

    // click on other pawn
    tick(g, mouse_click_tile(pos2));
    expect_eq!(g.selected_pawn_ids().sorted().collect_vec(), vec![crab2].sorted(), "clicked pawn should be selected");

    // deselect by clicking elsewhere
    tick(g, mouse_click_tile(pos2 + 12));
    expect_eq!(g.selected_pawn_ids().sorted().collect_vec(), vec![].sorted(), "should de-select");
}

#[gtest]
/// Select pawns by dragging a rectangle.
fn drag_selects_pawn() {
    let g = &mut small_world(caller!());
    g.ui.active_tool = Tool::Pointer;

    // add two pawns
    let pos1 = vec2(10, 11);
    let crab1 = g.spawn_pawn(Pawn::new(PawnTyp::Crablet, pos1, g.player)).id;

    let pos2 = vec2(13, 14);
    let crab2 = g.spawn_pawn(Pawn::new(PawnTyp::Crablet, pos2, g.player)).id;

    tick(g, [mouse_move_tile((9, 9))]);
    tick(g, [mouse_down()]);
    tick(g, [mouse_move_tile((15, 16))]);
    tick(g, [mouse_up()]);

    expect_eq!(g.selected_pawn_ids().sorted().collect_vec(), vec![crab1, crab2].sorted(), "drag to select");
}

/// Right-click moves selected pawn to destination.
#[gtest]
fn command_pawn_move() {
    let g = &mut small_world(caller!());

    let start = vec2(6, 7);
    let crab = g.spawn_pawn(Pawn::new(PawnTyp::Crablet, start, g.player)).id;
    g.select_pawn(crab);

    let dst = start + vec2(2, 2);
    tick(g, [mouse_move_tile(dst)]);
    tick(g, [key_down(K_MOUSE2)]);
    tick(g, [key_up(K_MOUSE2)]);
    tick(g, []);

    expect_eq!(g.pawn(crab).unwrap().destination(), Some(dst), "destination has been set");

    tick_n(g, 4);

    let pos = g.pawn(crab).unwrap().tile.get();
    expect_eq!(pos, dst, "pawn moved to destination");
}
