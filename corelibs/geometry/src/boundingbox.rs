use crate::internal::*;

pub trait MyNumber: Sized + Copy + PartialOrd + PartialEq + Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self> + Div<Output = Self> + Display + Debug + 'static {
    fn partial_min(self, other: Self) -> Self;
    fn partial_max(self, other: Self) -> Self;
    const ZERO: Self;
    const ONE: Self;
}

impl MyNumber for f32 {
    #[inline]
    fn partial_min(self, other: Self) -> Self {
        Self::min(self, other)
    }
    #[inline]
    fn partial_max(self, other: Self) -> Self {
        Self::max(self, other)
    }
    const ZERO: Self = 0.0;
    const ONE: Self = 1.0;
}

impl MyNumber for f64 {
    #[inline]
    fn partial_min(self, other: Self) -> Self {
        Self::min(self, other)
    }
    #[inline]
    fn partial_max(self, other: Self) -> Self {
        Self::max(self, other)
    }
    const ZERO: Self = 0.0;
    const ONE: Self = 1.0;
}

impl MyNumber for i32 {
    #[inline]
    fn partial_min(self, other: Self) -> Self {
        Ord::min(self, other)
    }
    #[inline]
    fn partial_max(self, other: Self) -> Self {
        Ord::max(self, other)
    }
    const ZERO: Self = 0;
    const ONE: Self = 1;
}

impl MyNumber for u32 {
    #[inline]
    fn partial_min(self, other: Self) -> Self {
        Ord::min(self, other)
    }
    #[inline]
    fn partial_max(self, other: Self) -> Self {
        Ord::max(self, other)
    }
    const ZERO: Self = 0;
    const ONE: Self = 1;
}

impl MyNumber for i8 {
    #[inline]
    fn partial_min(self, other: Self) -> Self {
        Ord::min(self, other)
    }
    #[inline]
    fn partial_max(self, other: Self) -> Self {
        Ord::max(self, other)
    }
    const ZERO: Self = 0;
    const ONE: Self = 1;
}

impl MyNumber for u8 {
    #[inline]
    fn partial_min(self, other: Self) -> Self {
        Ord::min(self, other)
    }
    #[inline]
    fn partial_max(self, other: Self) -> Self {
        Ord::max(self, other)
    }
    const ZERO: Self = 0;
    const ONE: Self = 1;
}

/// 'Any' floating point type. I.e. f32 or f64.
pub trait MyFloat: num_traits::Float + MyNumber + Neg<Output = Self> {
    fn sqrt(self) -> Self;
    fn is_finite(self) -> bool;
    fn as_f64(self) -> f64;
    const INF: Self;
}

impl MyFloat for f32 {
    #[inline]
    fn sqrt(self) -> Self {
        self.sqrt()
    }
    fn is_finite(self) -> bool {
        self.is_finite()
    }
    fn as_f64(self) -> f64 {
        self as f64
    }
    const INF: Self = Self::INFINITY;
}

impl MyFloat for f64 {
    #[inline]
    fn sqrt(self) -> Self {
        self.sqrt()
    }
    fn is_finite(self) -> bool {
        self.is_finite()
    }
    fn as_f64(self) -> f64 {
        self
    }
    const INF: Self = Self::INFINITY;
}

/// Axis Aligned Box, used to accelerate intersection tests with groups of objects.
/// See https://en.wikipedia.org/wiki/Minimum_bounding_box#Axis-aligned_minimum_bounding_box.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Eq)]
pub struct BoundingBox<T>
where
    T: Copy,
{
    pub min: vec3<T>,
    pub max: vec3<T>,
}

impl<T> BoundingBox<T>
where
    T: Copy + PartialOrd + Debug,
{
    /// Bounding box containing all points with coordinates between `min` and `max`.
    /// `min`'s components must not be larger than `max`'s.
    #[inline]
    pub fn new(min: vec3<T>, max: vec3<T>) -> Self {
        #[cfg(debug_assertions)]
        if !(min.zip_with(max, |min, max| (min, max)).iter().all(|(min, max)| min <= max)) {
            panic!("invalid bounding box: min: {min:?}, max: {max:?}")
        }
        Self { min, max }
    }
}

impl<T> BoundingBox<T>
where
    T: Copy,
{
    pub fn vertices(&self) -> impl Iterator<Item = vec3<T>> {
        let min = self.min;
        let max = self.max;
        [
            vec3(min.x(), min.y(), min.z()),
            vec3(min.x(), min.y(), max.z()),
            vec3(min.x(), max.y(), min.z()),
            vec3(min.x(), max.y(), max.z()),
            vec3(max.x(), min.y(), min.z()),
            vec3(max.x(), min.y(), max.z()),
            vec3(max.x(), max.y(), min.z()),
            vec3(max.x(), max.y(), max.z()),
        ]
        .into_iter()
    }
}

impl<T> BoundingBox<T>
where
    T: Copy,
{
    /// Convert the bounding box's vertex positions to a different type.
    /// ```
    /// # use geometry::*;
    /// # use vector::*;
    /// let bb = BoundingBox::new(vec3(1.0, 2.0, 3.0), vec3(4.0, 5.0, 6.0));
    /// assert_eq!(bb.convert::<i32>(), BoundingBox::new(vec3i(1, 2, 3), vec3i(4, 5, 6)));
    ///
    /// ```
    pub fn convert<U>(&self) -> BoundingBox<U>
    where
        T: AsPrimitive<U> + Debug,
        U: Copy + PartialOrd + Debug + 'static,
    {
        BoundingBox::new(self.min.as_(), self.max.as_())
    }
}

impl<T> BoundingBox<T>
where
    T: MyFloat,
{
    /// Center position.
    /// ```
    /// # use geometry::*;
    /// # use vector::*;
    /// let bb = BoundingBox::new(vec3(1.0, 2.0, 3.0), vec3(4.0, 5.0, 6.0));
    /// assert_eq!(bb.center(), vec3(2.5, 3.5, 4.5));
    /// ```
    pub fn center(&self) -> vec3<T> {
        (self.min + self.max) / (T::ONE + T::ONE)
    }

    pub fn center_bottom(&self) -> vec3<T> {
        self.center().with(|c| c[1] = self.min.y())
    }

    pub fn overlaps(&self, other: &Self) -> bool {
        // TODO: proper
        self.contains(other.center()) || other.contains(self.center())
    }
}

impl<T> BoundingBox<T>
where
    T: MyNumber,
{
    /// Size in each direction.
    /// ```
    /// # use geometry::*;
    /// # use vector::*;
    /// let bb = BoundingBox::new(vec3i(1, 2, 3), vec3i(2, 4, 8));
    /// assert_eq!(bb.size(), vec3i(1, 2, 5));
    /// ```
    pub fn size(&self) -> vec3<T> {
        self.max - self.min
    }

    /// Construct a bounding box enclosing `self` and an added point.
    /// ```
    /// # use geometry::*;
    /// # use vector::*;
    /// let a = BoundingBox::new(vec3i(1, 2, 3), vec3i(4, 5, 6));
    /// let b = a.add(vec3i(-1, 20, 4));
    /// assert_eq!(b, BoundingBox::new(vec3i(-1, 2, 3), vec3i(4, 20, 6)));
    /// ```
    #[must_use]
    pub fn add(&self, rhs: vec3<T>) -> Self {
        Self::new(self.min.zip_with(rhs, T::partial_min), self.max.zip_with(rhs, T::partial_max))
    }

    /// Construct a bounding box enclosing `self` and `rhs`.
    /// ```
    /// # use geometry::*;
    /// # use vector::*;
    /// let a = BoundingBox::new(vec3(1.0, 2.0, 3.0), vec3(4.0, 5.0, 6.0));
    /// let b = BoundingBox::new(vec3(-1.0, 5.0, 4.0), vec3(40.0, 20.0, 6.0));
    /// assert_eq!(a.join(&b), BoundingBox::new(vec3(-1.0, 2.0, 3.0), vec3(40.0, 20.0, 6.0)));
    /// ```
    #[must_use]
    pub fn join(&self, rhs: &Self) -> Self {
        Self::new(self.min.zip_with(rhs.min, T::partial_min), self.max.zip_with(rhs.max, T::partial_max))
    }

    /// Construct a bounding box enclosing all points from an iterator.
    /// ```
    /// # use geometry::*;
    /// # use vector::*;
    /// let points = [vec3f(1.0, 2.0, 3.0), vec3f(4.0, 5.0, 6.0), vec3f(-1.0, 20.0, 4.0)];
    /// let bb = BoundingBox::from_points(points.into_iter());
    /// assert_eq!(bb, Some(BoundingBox::new(vec3f(-1.0, 2.0, 3.0), vec3f(4.0, 20.0, 6.0))));
    ///
    /// let points = Vec::<vec3f>::new();
    /// let bb = BoundingBox::from_points(points.into_iter());
    /// assert_eq!(bb, None);
    /// ```
    pub fn from_points<'i, V: ToOwned<Owned = vec3<T>>>(mut vertices: impl Iterator<Item = V> + 'i) -> Option<Self> {
        let first = match vertices.next() {
            None => return None,
            Some(v) => v.to_owned(),
        };
        let mut bb = Self::new(first, first);
        for v in vertices {
            bb = bb.add(v.to_owned());
        }
        Some(bb)
    }

    /// Enclose all bounding boxes produced by an iterator.
    pub fn union(mut others: impl Iterator<Item = Self>) -> Option<Self> {
        let mut bb = match others.next() {
            None => return None,
            Some(v) => v.to_owned(),
        };
        for other in others {
            bb = bb.join(&other)
        }
        Some(bb)
    }

    /// Test if a point lies inside the bounding box
    /// (including its boundaries).
    /// ```
    /// # use geometry::*;
    /// # use vector::*;
    /// let bb = BoundingBox::new(vec3i(1,2,3), vec3i(4,5,6));
    /// assert_eq!(bb.contains(vec3i(1,2,3)), true);
    /// assert_eq!(bb.contains(vec3i(4,5,6)), true);
    /// assert_eq!(bb.contains(vec3i(2,4,4)), true);
    /// assert_eq!(bb.contains(vec3i(2,4,9)), false);
    /// assert_eq!(bb.contains(vec3i(2,9,4)), false);
    /// assert_eq!(bb.contains(vec3i(9,4,4)), false);
    /// assert_eq!(bb.contains(vec3i(-1,4,4)), false);
    /// assert_eq!(bb.contains(vec3i(2,-1,4)), false);
    /// assert_eq!(bb.contains(vec3i(2,4,-1)), false);
    /// ```
    pub fn contains(&self, point: vec3<T>) -> bool {
        point.x() >= self.min.x() //.
		&& point.x() <= self.max.x()
		&& point.y() >= self.min.y()
		&& point.y() <= self.max.y()
		&& point.z() >= self.min.z()
		&& point.z() <= self.max.z()
    }

    /// Like `contains` but considers the bounding box an open interval
    /// (so a point is not contained if it falls right on an edge).
    pub fn contains_excl(&self, point: vec3<T>) -> bool {
        point.x() > self.min.x() //.
		&& point.x() < self.max.x()
		&& point.y() > self.min.y()
		&& point.y() < self.max.y()
		&& point.z() > self.min.z()
		&& point.z() < self.max.z()
    }
}

pub type BoundingBox64 = BoundingBox<f64>;
pub type BoundingBox32 = BoundingBox<f32>;

impl<T> BoundingBox<T>
where
    T: MyFloat,
{
    #[inline]
    pub fn intersects(&self, r: &Ray<T>) -> bool {
        self.intersect(r).is_some()
    }

    #[inline]
    pub fn intersect(&self, r: &Ray<T>) -> Option<T> {
        let tmin = (self.min - r.start) / (r.dir);
        let tmax = (self.max - r.start) / (r.dir);

        let ten3 = tmin.zip_with(tmax, T::partial_min);
        let tex3 = tmin.zip_with(tmax, T::partial_max);

        let ten = ten3.reduce(T::partial_max);
        let tex = tex3.reduce(T::partial_min);

        // `>=` aims to cover the degenerate case where
        // the box has size 0 along a dimension
        // (e.g. when wrapping an axis-aligned rectangle).
        if tex >= T::partial_max(T::ZERO, ten) { Some(ten) } else { None }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EX: vec3d = vec3d::EX;
    const EY: vec3d = vec3d::EY;
    const EZ: vec3d = vec3d::EZ;

    fn ray(start: (f64, f64, f64), dir: vec3d) -> Ray64 {
        Ray64::new(vec3d::from(start), dir)
    }

    #[test]
    fn intersect() {
        let min = vec3d(1.0, 2.0, 3.0);
        let max = vec3d(2.0, 5.0, 6.0);
        let bb = BoundingBox64::new(min, max);

        /*
            Cases with the ray along X:

            <-(14)  (13)->     <-(16) (15)->   <-(18) (17)->

                              +-----------+(2,5,6)
                              |           |
                              |           |
            <-(2)  (1)->      |<-(4) (3)->|  <-(6) (5)->
                              |           |
                              |           |
                       (1,2,3)+-----------+

            <-(8)  (7)->       <-(9) (10)->   <-(12) (11)->
        */
        assert!(bb.intersects(&ray((0.0, 3.0, 4.0), EX))); //   (1)
        assert!(!bb.intersects(&ray((0.0, 3.0, 4.0), -EX))); // (2)
        assert!(bb.intersects(&ray((1.5, 3.0, 4.0), EX))); //   (3)
        assert!(bb.intersects(&ray((1.5, 3.0, 4.0), -EX))); //  (4)
        assert!(!bb.intersects(&ray((2.5, 3.0, 4.0), EX))); //  (5)
        assert!(bb.intersects(&ray((2.5, 3.0, 4.0), -EX))); //  (6)

        // as above, but shifted down (Y) to miss the box.
        assert!(!bb.intersects(&ray((0.0, -1.0, 4.0), EX))); // (7)
        assert!(!bb.intersects(&ray((0.0, -1.0, 4.0), -EX))); //(8)
        assert!(!bb.intersects(&ray((1.5, -1.0, 4.0), EX))); // (9)
        assert!(!bb.intersects(&ray((1.5, -1.0, 4.0), -EX))); //(10)
        assert!(!bb.intersects(&ray((2.5, -1.0, 4.0), EX))); // (11)
        assert!(!bb.intersects(&ray((2.5, -1.0, 4.0), -EX))); //(12)

        // as above, but shifted up (Y) to miss the box.
        assert!(!bb.intersects(&ray((0.0, 6.0, 4.0), EX))); // (13)
        assert!(!bb.intersects(&ray((0.0, 6.0, 4.0), -EX))); //(14)
        assert!(!bb.intersects(&ray((1.5, 6.0, 4.0), EX))); // (15)
        assert!(!bb.intersects(&ray((1.5, 6.0, 4.0), -EX))); //(16)
        assert!(!bb.intersects(&ray((2.5, 6.0, 4.0), EX))); // (17)
        assert!(!bb.intersects(&ray((2.5, 6.0, 4.0), -EX))); //(18)

        /*
            Cases with the ray along Y:

                                   ^
                                   |
                                  (6)
                                  (5)
                                   |
                                   v
                              +-----------+(2,5,6)
                              |    ^      |
                              |    |      |
                              |   (3)     |
                              |   (4)     |
                              |    |      |
                              |    v      |
                       (1,2,3)+-----------+
                                    ^
                                    |
                                   (1)
                                   (2)
                                    |
                                    v

        */
        assert!(bb.intersects(&ray((1.5, 1.0, 4.0), EY))); //   (1)
        assert!(!bb.intersects(&ray((1.5, 1.0, 4.0), -EY))); // (2)
        assert!(bb.intersects(&ray((1.5, 3.0, 4.0), EY))); //   (3)
        assert!(bb.intersects(&ray((1.5, 3.0, 4.0), -EY))); //  (4)
        assert!(bb.intersects(&ray((1.5, 6.0, 4.0), -EY))); //  (5)
        assert!(!bb.intersects(&ray((1.5, 6.0, 4.0), EY))); //  (6)

        // as above, but shifted left to miss the box
        assert!(!bb.intersects(&ray((0.5, 1.0, 4.0), EY)));
        assert!(!bb.intersects(&ray((0.5, 1.0, 4.0), -EY)));
        assert!(!bb.intersects(&ray((0.5, 3.0, 4.0), EY)));
        assert!(!bb.intersects(&ray((0.5, 3.0, 4.0), -EY)));
        assert!(!bb.intersects(&ray((0.5, 6.0, 4.0), -EY)));
        assert!(!bb.intersects(&ray((0.5, 6.0, 4.0), EY)));

        // as above, but shifted right to miss the box
        assert!(!bb.intersects(&ray((3.0, 1.0, 4.0), EY)));
        assert!(!bb.intersects(&ray((3.0, 1.0, 4.0), -EY)));
        assert!(!bb.intersects(&ray((3.0, 3.0, 4.0), EY)));
        assert!(!bb.intersects(&ray((3.0, 3.0, 4.0), -EY)));
        assert!(!bb.intersects(&ray((3.0, 6.0, 4.0), -EY)));
        assert!(!bb.intersects(&ray((3.0, 6.0, 4.0), EY)));

        // as above, but shifted right to miss the box
        assert!(!bb.intersects(&ray((3.0, 1.0, 4.0), EY)));
        assert!(!bb.intersects(&ray((3.0, 1.0, 4.0), -EY)));
        assert!(!bb.intersects(&ray((3.0, 3.0, 4.0), EY)));
        assert!(!bb.intersects(&ray((3.0, 3.0, 4.0), -EY)));
        assert!(!bb.intersects(&ray((3.0, 6.0, 4.0), -EY)));
        assert!(!bb.intersects(&ray((3.0, 6.0, 4.0), EY)));

        // Similar cases with the ray along Z:
        assert!(bb.intersects(&ray((1.5, 3.0, 2.0), EZ)));
        assert!(!bb.intersects(&ray((1.5, 3.0, 2.0), -EZ)));
        assert!(bb.intersects(&ray((1.5, 3.0, 4.0), EZ)));
        assert!(bb.intersects(&ray((1.5, 3.0, 4.0), -EZ)));
        assert!(bb.intersects(&ray((1.5, 3.0, 7.0), -EZ)));
        assert!(!bb.intersects(&ray((1.5, 3.0, 7.0), EZ)));

        // as above, but shifted to miss the box
        assert!(!bb.intersects(&ray((-1.0, 3.0, 2.0), EZ)));
        assert!(!bb.intersects(&ray((-1.0, 3.0, 2.0), -EZ)));
        assert!(!bb.intersects(&ray((-1.0, 3.0, 4.0), EZ)));
        assert!(!bb.intersects(&ray((-1.0, 3.0, 4.0), -EZ)));
        assert!(!bb.intersects(&ray((-1.0, 3.0, 7.0), -EZ)));
        assert!(!bb.intersects(&ray((-1.0, 3.0, 7.0), EZ)));
    }

    #[test]
    fn degenerate() {
        // Corner case: bounding box with size zero in one dimension.
        // It should still work (e.g.: this may happen when bounding a 2D shape).
        let bb = BoundingBox64::new(vec3d(-1., -1., 0.), vec3d(1., 1., 0.));
        assert!(bb.intersects(&ray((0., 0., 1.), -EZ)));
    }
}
