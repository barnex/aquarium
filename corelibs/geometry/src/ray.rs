//use std::sync::atomic::AtomicBool;

use crate::internal::*;

/// A `Ray` is a half-line defined by a starting point
/// and direction (unit vector).
/// Positions along the `Ray` are measured by their distance
/// `t` from the start:
///
///   start
///     +------|-------|-------|------->
///    t=0    t=1     t=2     t=3
///   
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ray<T>
where
	T: Float,
{
	pub start: vec3<T>,
	pub dir: vec3<T>,
}

impl<T> Ray<T>
where
	T: Float,
{
	/// Constructs a ray with given starting point and direction.
	/// Both must be finite, and dir must be a unit vector.
	#[inline]
	pub fn new(start: vec3<T>, dir: vec3<T>) -> Self {
		Self { start, dir }
	}

	/// The ray with its starting point offset by `delta_t` along the ray direction.
	#[must_use]
	#[inline]
	pub fn offset(&self, delta_t: T) -> Self {
		Self::new(self.start + self.dir * delta_t, self.dir)
	}

	/// Point at distance `t` (positive) from the start.
	#[inline]
	pub fn at(&self, t: T) -> vec3<T> {
		self.start + self.dir * t
	}
}

impl<T> Ray<T>
where
	T: Float,
{
	pub fn convert<U>(&self) -> Ray<U>
	where
		T: AsPrimitive<U> + 'static,
		U: Float + 'static,
	{
		Ray::new(self.start.as_(), self.dir.as_())
	}
}

pub type Ray64 = Ray<f64>;
pub type Ray32 = Ray<f32>;

impl Ray32 {
	pub fn as_f64(&self) -> Ray<f64> {
		self.convert()
	}
}
