use vector::*;

/// Pack rectangles of given sizes into a square.
/// Return each rectangle's position inside the square.
/// E.g.
/// input: `[((16,16), "sprite1"), ((16,16) "sprite2"), ((32, 32), "bigsprite")]`
///
/// output: `[("bigsprite", (0,0)), ("sprite1", (32, 0)), ("sprite2", (32, 16))]`
///
/// ```art
/// +--------------+
/// | big     | sp1|
/// |sprite   +----+
/// |         | sp2|
/// +---------+----+
/// ```
///
/// Rectangle IDs can be of any type.
pub fn binpack2d<ID>(size: u32, sizes: impl Iterator<Item = (vec2u, ID)>) -> impl Iterator<Item = (ID, vec2u)> {
    let rectangles = sizes.map(|(size, id)| Rectangle { size, id }).collect();
    let mut allocator = LightmapAllocator::new(size);
    let allocated = allocator.alloc_all(rectangles);
    allocated.into_iter().map(|(rectangle, pos)| (rectangle.id, pos))
}

struct LightmapAllocator {
    size: u32,
    curr: vec2u,
    next_y: u32,
}

#[derive(PartialEq, Eq, Clone, Hash, Debug)]
struct Rectangle<ID> {
    pub size: vec2u,
    pub id: ID,
}

/// Additional margin between islands.
/// An absolute minimum of 1 is needed to avoid light bleeding.
/// 1 additional texel is added to avoid minuscule light bleeding due to round-off under grazing incidence
/// (see fn VoxelWorld::add_borders).
const MARGIN: u32 = 2;

impl LightmapAllocator {
    fn new(size: u32) -> Self {
        Self { size, curr: vec2(1, 1), next_y: 1 }
    }

    /// Call `alloc` on each face.
    /// Does not necessarily conserve the original ordering.
    fn alloc_all<ID>(&mut self, mut faces: Vec<Rectangle<ID>>) -> Vec<(Rectangle<ID>, vec2u)> {
        faces.sort_by_key(|r| r.size.y()); // reduces lightmap fullness 3-10x
        faces
            .into_iter()
            .map(move |face| {
                let uv = self.alloc(face.size);
                (face, uv)
            })
            .collect()
    }

    /// Allocate a `(W+1)x(H+1)` island for a `WxH` face.
    ///
    /// texel 0   1   2   3   4   5   6...
    /// .   0 .   .   .   .   .   .   .   
    /// .       +---+       +-------+     
    /// .   1 . | . | .   . | .   . | .   
    /// .       +---+       |       |     
    /// .   2 .   .   .   . | .   . | .   
    /// .                   +-------+        
    /// .   3 .   .   .   .   .   .   .   
    /// .                                 
    /// .   4 .   .   .   .   .   .   .   
    fn alloc(&mut self, size: vec2u) -> vec2u {
        debug_assert!(size.x() > 0 && size.y() > 0);

        let size = size + vec2(1, 1);

        if self.curr.x() + size.x() >= self.size {
            // next line
            self.curr[0] = MARGIN;
            self.curr[1] = self.next_y;
        }

        self.next_y = u32::max(self.next_y, self.curr.y() + size.y() + MARGIN);

        let result = self.curr;
        self.curr[0] += size.x() + MARGIN;
        result
    }

    /*
    pub fn fullness(&self) -> f32 {
        (self.current.y as f32) / (self.size as f32)
    }
    */
}
