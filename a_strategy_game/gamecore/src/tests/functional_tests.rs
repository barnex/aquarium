//! Tests that don't require UI interaction.
use super::*;
use crate::*;
use googletest::prelude::*;

#[gtest]
fn deliver_resource_to_hq() {
    let g = &mut world_with_resources(caller!());
    g.debug.show_destination = false;
    g.debug.show_home = true;

    let hq = g.spawn_building(Building::new(BuildingTyp::HQ, vec2(5, 6), g.player)).unwrap();
    let crab = g.spawn(PawnTyp::Crablet, vec2(8, 7), g.player);

    g.assign_to(crab, hq);

    tick(g, []);
}
