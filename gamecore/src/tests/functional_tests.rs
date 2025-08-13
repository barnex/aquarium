//! Tests that don't require UI interaction.
use super::*;
use crate::*;
use googletest::prelude::*;


#[gtest]
fn deliver_resource_to_hq(){
    let g = &mut world_with_resources(test_name!());
	g.debug.show_destination = false;
	g.debug.show_home = true;

    let hq = g.spawn_building(Building::new(BuildingTyp::HQ, vec2(5, 6))).unwrap();
	let crab = g.spawn(Pawn::crab((8,7)));

	g.assign_to(crab, hq);

	tick(g, []);

}