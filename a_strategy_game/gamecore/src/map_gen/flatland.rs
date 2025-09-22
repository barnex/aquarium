use crate::prelude::*;

pub fn flatland() -> G{
        let mut g = G::new(vec2(480, 320));

        let hq = g.spawn_building(Building::new(BuildingTyp::HQ, (12, 8))).unwrap();

        g.spawn(Pawn::new(PawnTyp::Kitten, vec2(17, 7)));
        let crab = g.spawn(Pawn::new(PawnTyp::Crablet, vec2(10, 4)).with(|p| p.cargo = Some(ResourceTyp::Leaf).cel()));
        g.assign_to(crab, hq);

        let crab2 = g.spawn(Pawn::new(PawnTyp::Crablet, vec2(11, 5)).with(|p| p.cargo = Some(ResourceTyp::Leaf).cel()));
        g.assign_to(crab2, hq);

        g.spawn_resource((3, 9), ResourceTyp::Leaf);
        g.spawn_resource((7, 19), ResourceTyp::Rock);
        g.spawn_resource((17, 9), ResourceTyp::Rock);
        g.spawn_resource((15, 12), ResourceTyp::Leaf);
        g.spawn_resource((15, 13), ResourceTyp::Leaf);
        g.spawn_resource((16, 13), ResourceTyp::Leaf);
        g.spawn_resource((16, 12), ResourceTyp::Leaf);
        g.spawn_resource((17, 12), ResourceTyp::Leaf);
        g.spawn_resource((17, 18), ResourceTyp::Leaf);

        g
}