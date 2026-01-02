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

pub fn weighted_path_with_diag(g: &G, start: vec2i16, goal: vec2i16, max_dist: u16, walkable: impl Fn(vec2i16) -> bool) -> Option<Vec<vec2i16>> {
    use pathfinding::prelude::astar;
    let max_dist_square = (max_dist as i32).pow(2);
    let cost = |p| g.tile_at(p).distance_cost();
    let result = astar(
        &start,
        /* neighbors and distance: */
        //🪲 incorrect for diagonal moves
        |&cursor| {
            neighbors8dist(g, cursor)
                .into_iter()
                .filter(|&(p, _)| walkable(p) && start.distance_squared(p) < max_dist_square)
                .map(|(p, dist)| (p, dist + cost(p) as i32))
        },
        /* heuristic: */
        |&pos| (pos.as_f64().distance_to(goal.as_f64()) + cost(pos) as f64) as i32, // TODO: is this efficient?
        |&pos| pos == goal,
    );
    result.map(|(path, _dist)| path)
}

pub fn neighbors4(pos: vec2i16) -> [vec2i16; 4] {
    [(-1, 0), (0, -1), (1, 0), (0, 1)].map(|v| pos + vec2::from(v))
}

pub fn neighbors8(g: &G, pos: vec2i16) -> impl Iterator<Item = vec2i16> {
    let center = g.tile_at(pos);
    let mut diag = SmallVec::<vec2i16, 4>::new();

    for (d1, d2) in [(-1, 0), (0, -1), (1, 0), (0, 1)].into_iter().circular_tuple_windows() {
        if g.tile_at(pos + d1) == center || g.tile_at(pos + d2) == center {
            diag.push(pos + d1 + d2);
        }
    }

    neighbors4(pos).into_iter().chain(diag)
}

pub fn neighbors8dist(g: &G, pos: vec2i16) -> impl Iterator<Item = (vec2i16, i32)> {
    // We approximate diagonal distance between squares as 3/2 (i.e. 1.5, instead of the correct sqrt(2))
    const STRAIGHT_DIST: i32 = 2;
    const DIAG_DIST: i32 = 3;

    let center = g.tile_at(pos);
    let mut diag = SmallVec::<(vec2i16, i32), 4>::new();

    for (d1, d2) in [(-1, 0), (0, -1), (1, 0), (0, 1)].into_iter().circular_tuple_windows() {
        if g.tile_at(pos + d1) == center || g.tile_at(pos + d2) == center {
            diag.push((pos + d1 + d2, DIAG_DIST));
        }
    }

    neighbors4(pos).map(|p| (p, STRAIGHT_DIST)).into_iter().chain(diag)
}
