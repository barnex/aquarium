use crate::internal::*;
use std::cmp::PartialOrd;

pub type Bounds2Di = Bounds2D<i32>;
pub type Bounds2Du = Bounds2D<u32>;
pub type Bounds2Df = Bounds2D<f32>;

/// Axis Aligned Box, used to accelerate intersection tests with groups of objects.
/// See <https://en.wikipedia.org/wiki/Minimum_bounding_box#Axis-aligned_minimum_bounding_box>.
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize, Eq, Hash)]
pub struct Bounds2D<T>
where
    T: Copy,
{
    pub min: vec2<T>,
    pub max: vec2<T>,
}

impl<T> Bounds2D<T>
where
    T: Copy + PartialOrd + std::fmt::Debug,
{
    /// Bounding box containing all points with coordinates between `min` and `max`.
    /// `min`'s components must not be larger than `max`'s.
    #[inline]
    pub fn new(min: vec2<T>, max: vec2<T>) -> Self {
        debug_assert!(min.zip_with(max, |min, max| (min, max)).iter().all(|(min, max)| min <= max));
        Self { min, max }
    }

    /// The bounding box's 4 corners, in CCW order.
    pub fn corners(&self) -> [vec2<T>; 4] {
        let (x1, y1) = self.min.into();
        let (x2, y2) = self.max.into();
        [
            vec2(x1, y1), //
            vec2(x2, y1),
            vec2(x2, y2),
            vec2(x1, y2),
        ]
    }

    pub fn map<F, U>(&self, f: F) -> Bounds2D<U>
    where
        F: Fn(T) -> U + Copy,
        U: Copy,
    {
        Bounds2D { min: self.min.map(f), max: self.max.map(f) }
    }
}

impl<T> Bounds2D<T>
where
    T: Copy + PartialOrd + std::fmt::Debug + Add<Output = T>,
{
    #[inline]
    pub fn with_size(min: vec2<T>, size: vec2<T>) -> Self {
        Self::new(min, min + size)
    }
}

impl<T> Bounds2D<T>
where
    T: Copy + Add<Output = T>,
{
    pub fn from_pos_size(pos: vec2<T>, size: vec2<T>) -> Self {
        Self { min: pos, max: pos + size }
    }
}

impl<T> Bounds2D<T>
where
    T: Float,
{
    /// Center position.
    /// ```norun
    /// # use xxxxxxxxxxxxxxxx;
    /// let bb = Bounds2D::new(vec3f(1.0, 2.0, 3.0), vec3f(4.0, 5.0, 6.0));
    /// assert_eq!(bb.center(), vec3f(2.5, 3.5, 4.5));
    /// ```
    pub fn center(&self) -> vec2<T> {
        (self.min + self.max) / (T::one() + T::one())
    }
}

impl<T> Bounds2D<T>
where
    T: Number + Sub<Output = T> + PartialOrd,
{
    /// Size in each direction.
    /// ```norun
    /// # use xxxxxxxxxxxxxxxx;
    /// let bb = Bounds2D::new(vec3i(1, 2, 3), vec3i(2, 4, 8));
    /// assert_eq!(bb.size(), vec3i(1, 2, 5));
    /// ```
    pub fn size(&self) -> vec2<T> {
        self.max - self.min
    }

    /// Test if a point lies inside the bounding box
    /// (including its boundaries).
    pub fn contains(&self, point: vec2<T>) -> bool {
        point.x() >= self.min.x() //.
		&& point.x() <= self.max.x()
		&& point.y() >= self.min.y()
		&& point.y() <= self.max.y()
    }
}

impl Bounds2D<f32> {
    /// Does a line segment `start..end` intersect this bounding box?
    /// Used for conservative rasterization.
    /// ```norun
    /// # use xxxxxxxxxxxxxxxx;
    /// let bb = Bounds2D::new(vec3i(1, 2, 3), vec3i(2, 4, 8));
    /// assert_eq!(bb.size(), vec3i(1, 2, 5));
    /// ```
    #[inline]
    pub fn intersects_segment(&self, start: vec2f, end: vec2f) -> bool {
        let dir = (end - start).normalized();
        let tmin = (self.min - start) / (dir);
        let tmax = (self.max - start) / (dir);

        let ten3 = tmin.zip_with(tmax, f32::partial_min);
        let tex3 = tmin.zip_with(tmax, f32::partial_max);

        let ten = ten3.reduce(f32::partial_max);
        let tex = tex3.reduce(f32::partial_min);

        // `>=` aims to cover the degenerate case where
        // the box has size 0 along a dimension
        // (e.g. when wrapping an axis-aligned rectangle).
        if tex >= f32::partial_max(0.0, ten) { ten < (end - start).len() } else { false }
    }
}

impl Bounds2Di {
    pub fn intersect(&self, rhs: &Self) -> Self {
        Self {
            min: vec2(i32::max(self.min.x(), rhs.min.x()), i32::max(self.min.y(), rhs.min.y())),
            max: vec2(i32::min(self.max.x(), rhs.max.x()), i32::min(self.max.y(), rhs.max.y())),
        }
    }
}

impl Bounds2Du {
    pub fn intersect(&self, rhs: &Self) -> Self {
        Self {
            min: vec2(u32::max(self.min.x(), rhs.min.x()), u32::max(self.min.y(), rhs.min.y())),
            max: vec2(u32::min(self.max.x(), rhs.max.x()), u32::min(self.max.y(), rhs.max.y())),
        }
    }
}

impl Bounds2D<u32> {
    // Iterates over all points inside this rectangle, maximum *included*.
    pub fn iter_incl(&self) -> impl Iterator<Item = vec2u> + use<> {
        cross(self.min.x()..=self.max.x(), self.min.y()..=self.max.y()).map(|(x, y)| vec2(x, y))
    }
}

impl Bounds2D<i32> {
    // Iterates over all points inside this rectangle, maximum *included*.
    pub fn iter_incl(&self) -> impl Iterator<Item = vec2i> + use<> {
        cross(self.min.x()..=self.max.x(), self.min.y()..=self.max.y()).map(|(x, y)| vec2(x, y))
    }
}

impl Bounds2D<u16> {
    // Iterates over all points inside this rectangle, maximum *included*.
    pub fn iter_incl(&self) -> impl Iterator<Item = vec2u16> + use<> {
        cross(self.min.x()..=self.max.x(), self.min.y()..=self.max.y()).map(|(x, y)| vec2(x, y))
    }

    pub fn intersect(&self, rhs: &Self) -> Self {
        Self {
            min: vec2(u16::max(self.min.x(), rhs.min.x()), u16::max(self.min.y(), rhs.min.y())),
            max: vec2(u16::min(self.max.x(), rhs.max.x()), u16::min(self.max.y(), rhs.max.y())),
        }
    }
}
