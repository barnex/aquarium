use crate::internal::*;

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct DistanceMap {
    radius: u16,
    center: vec2i16,
    dist_to: HashMap<vec2i16, u16>,
}

impl DistanceMap {
    pub fn new(center: vec2i16, max_dist: u16, walkable: impl Fn(vec2i16) -> bool) -> Self {
        let mut dist_to = HashMap::<vec2i16, u16>::from_iter([(center, 0)]);
        let mut front = VecDeque::from([center]);

        let visited = |dist_to: &HashMap<_, _>, pos| dist_to.contains_key(&pos);
        let radius2 = (max_dist as i32) * (max_dist as i32);

        while let Some(pos) = front.pop_front() {
            let neigh_dist = dist_to[&pos] + 1;
            if neigh_dist <= (u8::MAX as u16) {
                for pos in neighbors(pos) {
                    if center.as_i32().distance_squared(pos.as_i32()) < radius2 && !visited(&dist_to, pos) && walkable(pos) {
                        dist_to.insert(pos, neigh_dist);
                        front.push_back(pos)
                    }
                }
            }
        }

        Self { center, radius: max_dist, dist_to }
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

    fn path_to_center(&self, start: vec2i16) -> Option<Vec<vec2i16>> {
        let mut curr_dist = *self.dist_to.get(&start)?; // 👈 returns `None` if unreachable
        let initial_dist = curr_dist;
        let mut path = Vec::with_capacity(curr_dist as usize + 1); // exact capacity
        path.push(start);
        let mut cursor = start;
        'next_tile: while curr_dist > 0 {
            for neigh in neighbors(cursor) {
                if let Some(&dist) = self.dist_to.get(&neigh) {
                    if dist < curr_dist {
                        curr_dist = dist;
                        cursor = neigh;
                        path.push(cursor);
                        continue 'next_tile;
                    }
                }
            }
        }

        debug_assert_eq!(path.len(), initial_dist as usize + 1); // check capacity was exact
        Some(path)
    }

    fn path_from_center(&self, destination: vec2i16) -> Option<Vec<vec2i16>> {
        self.path_to_center(destination).map(|v| v.with(|v| v.reverse()))
    }
}

fn neighbors(pos: vec2i16) -> [vec2i16; 4] {
    [(-1, 0), (1, 0), (0, -1), (0, 1)].map(|v| pos + vec2::from(v))
}
