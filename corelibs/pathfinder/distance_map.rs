use smallvec::SmallVec;

use crate::internal::*;

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

// TODO: quite inefficient. Use A* instead. Factories should compute one distance map and re-use for all paths to nearby points of interest.
//pub fn path_to(start: vec2i16, dest: vec2i16, max_dist: u16, walkable: impl Fn(vec2i16) -> bool) -> Option<Vec<vec2i16>> {
//    let distance_map = DistanceMap::new(dest, max_dist, walkable);
//    distance_map.path_to_center(start)
//}

/*
#[derive(Default, Serialize, Deserialize, Clone)]
pub struct DistanceMap {
    radius: u16,
    center: vec2i16,
    dist_to: HashMap<vec2i16, u16>,
}

impl DistanceMap {
    pub fn new_weighted(center: vec2i16, max_dist: u16, walkable: impl Fn(vec2i16) -> bool, weight: impl Fn(vec2i16) -> u8) -> Self {
        let mut dist_to = HashMap::<vec2i16, u16>::from_iter([(center, 0)]);
        let mut front = VecDeque::from([center]);

        let visited = |dist_to: &HashMap<_, _>, pos| dist_to.contains_key(&pos);
        let radius2 = (max_dist as i64) * (max_dist as i64);

        while let Some(pos) = front.pop_front() {
            let neigh_dist = dist_to[&pos] + (weight(pos).with(|w| debug_assert!(*w != 0)) as u16);
            if neigh_dist <= (u8::MAX as u16) {
                for pos in neighbors(pos) {
                    if center.as_i32().distance_squared(pos.as_i32()) < radius2 && !visited(&dist_to, pos) && walkable(pos) {
                        let current_best = dist_to.get(&pos).copied().unwrap_or(u16::MAX);
                        if neigh_dist < current_best {
                            dist_to.insert(pos, neigh_dist);
                        }
                        front.push_back(pos)
                    }
                }
            }
        }

        Self { center, radius: max_dist, dist_to }
    }

    pub fn new(center: vec2i16, max_dist: u16, walkable: impl Fn(vec2i16) -> bool) -> Self {
        Self::new_weighted(center, max_dist, walkable, |_| 1)
    }

    /// Iterate all reachable positions and distances.
    pub fn iter(&self) -> impl Iterator<Item = (vec2i16, u16)> {
        self.dist_to.iter().map(|(k, v)| (*k, *v))
    }

    /// Is there a path between `self.center` and position `pos`?
    pub fn is_reachable(&self, pos: vec2i16) -> bool {
        self.dist_to.contains_key(&pos)
    }

    pub fn distance_to(&self, pos: vec2i16) -> Option<u16> {
        self.dist_to.get(&pos).copied()
    }

    pub fn path_to_but_very_poor(&self, start: vec2i16, destination: vec2i16) -> Option<Vec<vec2i16>> {
        if self.dist_to.get(&destination)? == &0 {
            return self.path_to_center(start);
        }

        if self.dist_to.get(&start)? == &0 {
            return self.path_from_center(destination);
        }

        let to_center = self.path_to_center(start)?;
        let from_center = self.path_from_center(destination)?;
        Some(to_center.with(|v| v.extend_from_slice(&from_center)))
    }

    pub fn path_to_center(&self, start: vec2i16) -> Option<Vec<vec2i16>> {
        let mut cursor = start;
        let mut curr_dist = *self.dist_to.get(&start)?; // 👈 returns `None` if unreachable
        let mut path = Vec::with_capacity(curr_dist as usize + 1); // exact capacity🪲 unless weighted

        path.push(start);
        'next_tile: while curr_dist > 0 {
            eprintln!("*********************");
            dbg!((cursor, curr_dist));
            let neighbors = neighbors(cursor) //_
                .into_iter()
                .filter_map(|pos| self.dist_to.get(&pos).map(|dist| (pos, *dist)));
            //.inspect(|(pos, dist)| dbg!((pos, dist)).ignore())
            //.collect_vec() // << ☠️
            //.into_iter();

            for (neigh, dist) in neighbors.sorted_by_key(|(_, dist)| *dist) {
                //🪲 sorted_by_key is inefficient. use min_by_key?
                if dist < curr_dist {
                    curr_dist = dist;
                    cursor = neigh;
                    path.push(cursor);
                    continue 'next_tile;
                }
            }
        }

        //debug_assert_eq!(path.len(), initial_dist as usize + 1); // check capacity was exact
        Some(path)
    }

    fn path_from_center(&self, destination: vec2i16) -> Option<Vec<vec2i16>> {
        self.path_to_center(destination).map(|v| v.with(|v| v.reverse()))
    }
}
*/

fn neighbors4(pos: vec2i16) -> [vec2i16; 4] {
    [(-1, 0), (1, 0), (0, -1), (0, 1)].map(|v| pos + vec2::from(v))
}

//fn neighbors8(g: &G, pos: vec2i16) -> SmallVec<vec2i16, 8> {
//    let mut result = SmallVec::new();
//    for neigh in [(-1, 0), (1, 0), (0, -1), (0, 1)].map(|v| pos + vec2::from(v)) {}
//    result
//}
