use crate::prelude::*;

pub fn weighted_path_to(start: vec2i16, goal: vec2i16, max_dist: u16, walkable: impl Fn(vec2i16) -> bool, cost: impl Fn(vec2i16) -> u8) -> Option<Vec<vec2i16>> {
    use pathfinding::prelude::astar;
    let max_dist_square = (max_dist as i32).pow(2);
    let result = astar(
        &start,
        /* neighbors and distance: */
        |&cursor| neighbors4(cursor).into_iter().filter(|&p| walkable(p) && start.distance_squared(p) < max_dist_square).map(|p| (p, cost(p) as i32)),
        /* heuristic: */
        |&pos| (pos.as_f64().distance_to(goal.as_f64()) + cost(pos) as f64) as i32, // TODO: is this efficient?
        |&pos| pos == goal,
    );
    result.map(|(path, _dist)| path)
}

pub fn neighbors4(pos: vec2i16) -> [vec2i16; 4] {
    [(-1, 0), (1, 0), (0, -1), (0, 1)].map(|v| pos + vec2::from(v))
}
