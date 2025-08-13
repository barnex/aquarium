//! Tests that don't require UI interaction.
use super::*;
use crate::*;
use googletest::prelude::*;


#[gtest]
fn deliver_resource_to_hq(){
    let g = &mut world_with_hq(test_name!());

	g.spawn(Pawn::crab((5,7)));

	tick(g, []);



}