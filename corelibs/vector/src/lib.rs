#![allow(non_camel_case_types)]
#![deny(warnings)]

//! Crate provides 2, 3 and 4-component vector types similar to [WGSL](https://www.w3.org/TR/WGSL/).

//#[cfg(feature = "inspect")]
//mod inspect;

use bytemuck::{Pod, Zeroable};
use num_traits::AsPrimitive;
use num_traits::Float;
use serde::{Deserialize, Serialize};
use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign};

/// 2, 3 or 4-component vector similar to [WGSL](https://www.w3.org/TR/WGSL/) vector types.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct Vector<T, const N: usize>(pub [T; N]);

/// 2-component vector
pub type vec2<T> = Vector<T, 2>;
/// 3-component vector
pub type vec3<T> = Vector<T, 3>;
/// 4-component vector
pub type vec4<T> = Vector<T, 4>;

/// f32 vector
pub type vec2f = vec2<f32>;
/// f32 vector
pub type vec3f = vec3<f32>;
/// f32 vector
pub type vec4f = vec4<f32>;

/// f64 vector
pub type vec2d = vec2<f64>;
/// f64 vector
pub type vec3d = vec3<f64>;
/// f64 vector
pub type vec4d = vec4<f64>;

/// i32 vector
pub type vec2i = vec2<i32>;
/// i32 vector
pub type vec3i = vec3<i32>;
/// i32 vector
pub type vec4i = vec4<i32>;

/// u32 vector
pub type vec2u = vec2<u32>;
/// u32 vector
pub type vec3u = vec3<u32>;
/// u32 vector
pub type vec4u = vec4<u32>;

/// u8 vector
pub type vec2u8 = vec2<u8>;
/// u8 vector
pub type vec3u8 = vec3<u8>;
/// u8 vector
pub type vec4u8 = vec4<u8>;

/// u16 vector
pub type vec2u16 = vec2<u16>;
/// u16 vector
pub type vec3u16 = vec3<u16>;
/// u16 vector
pub type vec4u16 = vec4<u16>;

/// i16 vector
pub type vec2i16 = vec2<i16>;
/// i16 vector
pub type vec3i16 = vec3<i16>;
/// i16 vector
pub type vec4i16 = vec4<i16>;

/// Convenience constructors.
#[rustfmt::skip]
mod constructors {
	use super::*;

	/// Constructor
	#[inline(always)] pub const fn vec2<T>(x: T, y: T            ) -> vec2<T> { Vector([x, y]) }
	/// Constructor
	#[inline(always)] pub const fn vec3<T>(x: T, y: T, z: T      ) -> vec3<T> { Vector([x, y, z]) }
	/// Constructor
	#[inline(always)] pub const fn vec4<T>(x: T, y: T, z: T, w: T) -> vec4<T> { Vector([x, y, z, w]) }

	/// Constructor
	#[inline(always)] pub const fn vec2f(x: f32, y: f32                ) -> vec2f { Vector([x, y]) }
	/// Constructor
	#[inline(always)] pub const fn vec3f(x: f32, y: f32, z: f32        ) -> vec3f { Vector([x, y, z]) }
	/// Constructor
	#[inline(always)] pub const fn vec4f(x: f32, y: f32, z: f32, w: f32) -> vec4f { Vector([x, y, z, w]) }

	/// Constructor
	#[inline(always)] pub const fn vec2d(x: f64, y: f64                ) -> vec2d { Vector([x, y]) }
	/// Constructor
	#[inline(always)] pub const fn vec3d(x: f64, y: f64, z: f64        ) -> vec3d { Vector([x, y, z]) }
	/// Constructor
	#[inline(always)] pub const fn vec4d(x: f64, y: f64, z: f64, w: f64) -> vec4d { Vector([x, y, z, w]) }

	/// Constructor
	#[inline(always)] pub const fn vec2i(x: i32, y: i32                ) -> vec2i { Vector([x, y]) }
	/// Constructor
	#[inline(always)] pub const fn vec3i(x: i32, y: i32, z: i32        ) -> vec3i { Vector([x, y, z]) }
	/// Constructor
	#[inline(always)] pub const fn vec4i(x: i32, y: i32, z: i32, w: i32) -> vec4i { Vector([x, y, z, w]) }

	/// Constructor
	#[inline(always)] pub const fn vec2u(x: u32, y: u32                ) -> vec2u { Vector([x, y]) }
	/// Constructor
	#[inline(always)] pub const fn vec3u(x: u32, y: u32, z: u32        ) -> vec3u { Vector([x, y, z]) }
	/// Constructor
	#[inline(always)] pub const fn vec4u(x: u32, y: u32, z: u32, w: u32) -> vec4u { Vector([x, y, z, w]) }
    
	/// Constructor
	#[inline(always)] pub const fn vec2u8(x: u8, y: u8               ) -> vec2u8 { Vector([x, y]) }
	/// Constructor
	#[inline(always)] pub const fn vec3u8(x: u8, y: u8, z: u8        ) -> vec3u8 { Vector([x, y, z]) }
	/// Constructor
	#[inline(always)] pub const fn vec4u8(x: u8, y: u8, z: u8, w: u8)  -> vec4u8{ Vector([x, y, z, w]) }
    
	/// Constructor
	#[inline(always)] pub const fn vec2u16(x: u16, y: u16                ) -> vec2u16 { Vector([x, y]) }
	/// Constructor
	#[inline(always)] pub const fn vec3u16(x: u16, y: u16, z: u16        ) -> vec3u16 { Vector([x, y, z]) }
	/// Constructor
	#[inline(always)] pub const fn vec4u16(x: u16, y: u16, z: u16, w: u16) -> vec4u16 { Vector([x, y, z, w]) }
    
	/// Constructor
	#[inline(always)] pub const fn vec2i16(x: i16, y: i16                ) -> vec2i16 { Vector([x, y]) }
	/// Constructor
	#[inline(always)] pub const fn vec3i16(x: i16, y: i16, z: i16        ) -> vec3i16 { Vector([x, y, z]) }
	/// Constructor
	#[inline(always)] pub const fn vec4i16(x: i16, y: i16, z: i16, w: i16) -> vec4i16 { Vector([x, y, z, w]) }

	impl<T, const N: usize> Default for Vector<T, N>
	where
		[T; N]: Default,
	{
		fn default() -> Self {
			Self(<[T; N]>::default())
		}
	}

	impl<T: Copy, const N: usize> Vector<T, N> {
		/// All elements set to `v`.
		/// ```
		/// # use vector::*;
		/// assert_eq!(vec4::splat(6), vec4(6, 6, 6, 6));
		/// ```
		pub const fn splat(v: T) -> Self {
			Self([v; N])
		}
	}
}
pub use constructors::*;

/// Constants like `vec3::ONES`, `vec4::EZ`, ...
#[rustfmt::skip]
mod constants {
	use super::*;

	/// Internal-only trait implemented for all numbers.
	/// Used to provide constants like `vec3::EX`.
	/// Similar to `num_traits::Num` but provides *constants* `ZERO`, `ONE`.
	pub trait Number: Copy{
		const ZERO: Self;
		const ONE: Self;
	}

	impl Number for f32 { const ONE: Self = 1.0; const ZERO: Self = 0.0; }
	impl Number for f64 { const ONE: Self = 1.0; const ZERO: Self = 0.0; }
	impl Number for u8  { const ONE: Self = 1; const ZERO: Self = 0; }
	impl Number for u16 { const ONE: Self = 1; const ZERO: Self = 0; }
	impl Number for u32 { const ONE: Self = 1; const ZERO: Self = 0; }
	impl Number for u64 { const ONE: Self = 1; const ZERO: Self = 0; }
	impl Number for i8  { const ONE: Self = 1; const ZERO: Self = 0; }
	impl Number for i16 { const ONE: Self = 1; const ZERO: Self = 0; }
	impl Number for i32 { const ONE: Self = 1; const ZERO: Self = 0; }
	impl Number for i64 { const ONE: Self = 1; const ZERO: Self = 0; }

	impl<T: Number, const N: usize> Vector<T, N> { 
		/// All components set to 0.
		pub const ZERO: Self = Self::splat(T::ZERO); 
		/// All components set to 1.
		pub const ONES: Self = Self::splat(T::ONE); 
	}

	impl<T: Number> Vector<T, 2> { 
		/// X unit vector.
		pub const EX: Self = Self([T::ONE, T::ZERO]);
		/// Y unit vector.
		pub const EY: Self = Self([T::ZERO, T::ONE]);
		/// X and Y unit vectors.
		pub const UNIT: [Self; 2] = [Self::EX, Self::EY];
	}
	impl<T: Number> Vector<T, 3> { 
		/// X unit vector.
		pub const EX: Self = Self([T::ONE, T::ZERO, T::ZERO]);
		/// Y unit vector.
		pub const EY: Self = Self([T::ZERO, T::ONE, T::ZERO]);
		/// Z unit vector.
		pub const EZ: Self = Self([T::ZERO, T::ZERO, T::ONE]);
		/// X, Y, Z unit vectors.
		pub const UNIT: [Self; 3] = [Self::EX, Self::EY, Self::EZ];
	}
	impl<T: Number> Vector<T, 4> { 
		/// X unit vector.
		pub const EX: Self = Self([T::ONE, T::ZERO, T::ZERO, T::ZERO]);
		/// Y unit vector.
		pub const EY: Self = Self([T::ZERO, T::ONE, T::ZERO, T::ZERO]);
		/// Z unit vector.
		pub const EZ: Self = Self([T::ZERO, T::ZERO, T::ONE, T::ZERO]);
		/// W unit vector.
		pub const EW: Self = Self([T::ZERO, T::ZERO, T::ZERO, T::ONE]);
		/// X, Y, Z, W unit vectors.
		pub const UNIT: [Self; 4] = [Self::EX, Self::EY, Self::EZ, Self::EW];
	}

}
pub use constants::*;

/// [Swizzle](https://www.w3.org/TR/WGSL/#swizzle) methods like `xz`.
#[rustfmt::skip]
mod swizzle {
	use super::*;

	impl<T: Copy> vec2<T> {
		/// X component.
		#[inline(always)] pub const fn x(self) -> T { self.0[0] }
		/// Y component.
		#[inline(always)] pub const fn y(self) -> T { self.0[1] }
	}

	impl<T: Copy> vec3<T> {
		/// X component.
		#[inline(always)] pub const fn x(self) -> T { self.0[0] }
		/// Y component.
		#[inline(always)] pub const fn y(self) -> T { self.0[1] }
		/// Z component.
		#[inline(always)] pub const fn z(self) -> T { self.0[2] }

		/// Y and Z components
		#[inline(always)] pub const fn yz(self) -> vec2<T> { Vector([self.0[1], self.0[2]]) }
		/// X and Z components
		#[inline(always)] pub const fn xz(self) -> vec2<T> { Vector([self.0[0], self.0[2]]) }
		/// X and Y components
		#[inline(always)] pub const fn xy(self) -> vec2<T> { Vector([self.0[0], self.0[1]]) }
	}

	impl<T: Copy> vec4<T> {
		/// X component.
		#[inline(always)] pub const fn x(self) -> T { self.0[0] }
		/// Y component.
		#[inline(always)] pub const fn y(self) -> T { self.0[1] }
		/// Z component.
		#[inline(always)] pub const fn z(self) -> T { self.0[2] }
		/// W component.
		#[inline(always)] pub const fn w(self) -> T { self.0[3] }

		/// Y and Z components
		#[inline(always)] pub const fn yz(self) -> vec2<T> { Vector([self.0[1], self.0[2]]) }
		/// X and Z components
		#[inline(always)] pub const fn xz(self) -> vec2<T> { Vector([self.0[0], self.0[2]]) }
		/// X and Y components
		#[inline(always)] pub const fn xy(self) -> vec2<T> { Vector([self.0[0], self.0[1]]) }

		/// X, Y and Z components
		#[inline(always)] pub const fn xyz(self) -> vec3<T> { Vector([self.0[0], self.0[1], self.0[2]]) }
	}

	impl<T> vec2<T> { 
		/// Append Z component.
		/// ```
		/// # use vector::*;
		/// assert_eq!(vec2(1, 2).append(3), vec3(1, 2, 3));
		/// ```
		#[inline(always)]
		#[must_use]
		pub fn append(self, z: T) -> vec3<T> { let Vector([x, y]) = self;    Vector([x, y, z]) }
	 }
	impl<T> vec3<T> { 
		/// Append W component.
		/// ```
		/// # use vector::*;
		/// assert_eq!(vec3(1, 2, 3).append(4), vec4(1, 2, 3, 4));
		/// ```
		#[inline(always)]
		#[must_use]
		pub fn append(self, w: T) -> vec4<T> { let Vector([x, y, z]) = self; Vector([x, y, z, w]) }
	 }
}

/// Conversions between shapes and numerical types.
#[rustfmt::skip]
mod conversions{
	use super::*;

	impl<T, const N: usize> From<[T; N]>    for Vector<T, N> { fn from(inner: [T; N]) -> Self   { Self(inner) } }
	impl<T, const N: usize> From<Vector<T, N>> for [T; N]    { fn from(v: Vector<T, N>)  -> [T; N] { v.0 } }

	impl<T, const N: usize> Vector<T, N> { pub fn array(self) -> [T; N] { self.0 } }

	impl<T> From<(T, T)>       for vec2<T> { #[inline(always)] fn from((x, y):       (T, T))       -> Self { Self([x, y])       } }
	impl<T> From<(T, T, T)>    for vec3<T> { #[inline(always)] fn from((x, y, z):    (T, T, T))    -> Self { Self([x, y, z])    } }
	impl<T> From<(T, T, T, T)> for vec4<T> { #[inline(always)] fn from((x, y, z, w): (T, T, T, T)) -> Self { Self([x, y, z, w]) } }

	impl<T> Vector<T, 2> { pub fn tuple(self) -> (T, T)       { self.into() } }
	impl<T> Vector<T, 3> { pub fn tuple(self) -> (T, T, T)    { self.into() } }
	impl<T> Vector<T, 4> { pub fn tuple(self) -> (T, T, T, T) { self.into() } }

	impl<T> From<vec2<T>> for (T, T)       { #[inline(always)] fn from(Vector([x, y]): vec2<T>)       -> (T, T)       { (x, y)       } }
	impl<T> From<vec3<T>> for (T, T, T)    { #[inline(always)] fn from(Vector([x, y, z]): vec3<T>)    -> (T, T, T)    { (x, y, z)    } }
	impl<T> From<vec4<T>> for (T, T, T, T) { #[inline(always)] fn from(Vector([x, y, z, w]): vec4<T>) -> (T, T, T, T) { (x, y, z, w) } }


	impl<T, const N: usize> Vector<T, N> {
	    #[inline(always)]
	    pub fn as_<U>(self) -> Vector<U, N>
	    where
	        T: AsPrimitive<U>,
	        U: Copy + 'static,
	    {
	        self.map(|v| v.as_())
	    }
	}
	impl<T: AsPrimitive<f32>, const N: usize> Vector<T, N> { #[inline(always)] pub fn as_f32(self) -> Vector<f32, N> { self.map(|v| v.as_()) } }
	impl<T: AsPrimitive<f64>, const N: usize> Vector<T, N> { #[inline(always)] pub fn as_f64(self) -> Vector<f64, N> { self.map(|v| v.as_()) } }
	impl<T: AsPrimitive<i64>, const N: usize> Vector<T, N> { #[inline(always)] pub fn as_i64(self) -> Vector<i64, N> { self.map(|v| v.as_()) } }
	impl<T: AsPrimitive<i32>, const N: usize> Vector<T, N> { #[inline(always)] pub fn as_i32(self) -> Vector<i32, N> { self.map(|v| v.as_()) } }
	impl<T: AsPrimitive<i16>, const N: usize> Vector<T, N> { #[inline(always)] pub fn as_i16(self) -> Vector<i16, N> { self.map(|v| v.as_()) } }
	impl<T: AsPrimitive<i8 >, const N: usize> Vector<T, N> { #[inline(always)] pub fn as_i8 (self) -> Vector<i8 , N> { self.map(|v| v.as_()) } }
	impl<T: AsPrimitive<u64>, const N: usize> Vector<T, N> { #[inline(always)] pub fn as_u64(self) -> Vector<u64, N> { self.map(|v| v.as_()) } }
	impl<T: AsPrimitive<u32>, const N: usize> Vector<T, N> { #[inline(always)] pub fn as_u32(self) -> Vector<u32, N> { self.map(|v| v.as_()) } }
	impl<T: AsPrimitive<u16>, const N: usize> Vector<T, N> { #[inline(always)] pub fn as_u16(self) -> Vector<u16, N> { self.map(|v| v.as_()) } }
	impl<T: AsPrimitive<u8 >, const N: usize> Vector<T, N> { #[inline(always)] pub fn as_u8 (self) -> Vector<u8 , N> { self.map(|v| v.as_()) } }
	impl<T: AsPrimitive<usize>, const N: usize> Vector<T, N> { #[inline(always)] pub fn as_usize(self) -> Vector<usize, N> { self.map(|v| v.as_()) } }
}

impl<T, const N: usize> std::fmt::Debug for Vector<T, N>
where
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.0.iter()).finish()
    }
}

impl<T, const N: usize> std::fmt::Display for Vector<T, N>
where
    T: std::fmt::Display + std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.0.iter()).finish()
    }
}

/// Internal-only trait implemented for supported vector sizes (2, 3, 4).
/// Only needed because [`[T;N]`](https://doc.rust-lang.org/std/primitive.array.html),
/// is currently missing a few required methods like `zip`.
/// (Also see <https://doc.rust-lang.org/std/primitive.array.html#method.each_mut>).
pub trait SupportedSize<const N: usize> {
    fn zip<T, U>(a: Vector<T, N>, b: Vector<U, N>) -> Vector<(T, U), N>;
    fn reduce<T, F: Fn(T, T) -> T>(a: Vector<T, N>, f: F) -> T;
    fn zip_mut_with<T, U, F: Fn(&mut T, U)>(a: &mut Vector<T, N>, b: Vector<U, N>, f: F);

    #[inline(always)]
    fn for_each<T, F: Fn(&mut T)>(a: &mut Vector<T, N>, f: F) {
        Self::zip_mut_with(a, Vector::splat(()), |ptr, ()| f(ptr))
    }

    #[inline(always)]
    fn zip_with<T, U, V, F: Fn(T, U) -> V>(a: Vector<T, N>, b: Vector<U, N>, f: F) -> Vector<V, N> {
        Self::zip(a, b).map(|(a, b)| f(a, b))
    }
}

impl SupportedSize<2> for () {
    #[inline(always)]
    fn zip<T, U>(Vector([a0, a1]): Vector<T, 2>, Vector([b0, b1]): Vector<U, 2>) -> Vector<(T, U), 2> {
        Vector([(a0, b0), (a1, b1)])
    }

    #[inline(always)]
    fn zip_mut_with<T, U, F: Fn(&mut T, U)>(a: &mut Vector<T, 2>, Vector([b0, b1]): Vector<U, 2>, f: F) {
        f(&mut a[0], b0);
        f(&mut a[1], b1);
    }

    #[inline(always)]
    fn reduce<T, F: Fn(T, T) -> T>(Vector([a0, a1]): Vector<T, 2>, f: F) -> T {
        f(a0, a1)
    }
}

impl SupportedSize<3> for () {
    #[inline(always)]
    fn zip<T, U>(Vector([a0, a1, a2]): Vector<T, 3>, Vector([b0, b1, b2]): Vector<U, 3>) -> Vector<(T, U), 3> {
        Vector([(a0, b0), (a1, b1), (a2, b2)])
    }

    #[inline(always)]
    fn zip_mut_with<T, U, F: Fn(&mut T, U)>(a: &mut Vector<T, 3>, Vector([b0, b1, b2]): Vector<U, 3>, f: F) {
        f(&mut a[0], b0);
        f(&mut a[1], b1);
        f(&mut a[2], b2);
    }

    #[inline(always)]
    fn reduce<T, F: Fn(T, T) -> T>(Vector([a0, a1, a2]): Vector<T, 3>, f: F) -> T {
        f(f(a0, a1), a2)
    }
}

impl SupportedSize<4> for () {
    #[inline(always)]
    fn zip<T, U>(Vector([a0, a1, a2, a3]): Vector<T, 4>, Vector([b0, b1, b2, b3]): Vector<U, 4>) -> Vector<(T, U), 4> {
        Vector([(a0, b0), (a1, b1), (a2, b2), (a3, b3)])
    }

    #[inline(always)]
    fn zip_mut_with<T, U, F: Fn(&mut T, U)>(a: &mut Vector<T, 4>, Vector([b0, b1, b2, b3]): Vector<U, 4>, f: F) {
        f(&mut a[0], b0);
        f(&mut a[1], b1);
        f(&mut a[2], b2);
        f(&mut a[3], b3);
    }

    #[inline(always)]
    fn reduce<T, F: Fn(T, T) -> T>(Vector([a0, a1, a2, a3]): Vector<T, 4>, f: F) -> T {
        f(f(a0, a1), f(a2, a3))
    }
}

/// Iterator-like method, but with fixed sizes where appropriate.
mod iterator {
    use super::*;

    impl<T, const N: usize> Vector<T, N> {
        /// Iterate over the elements.
        #[inline]
        pub fn iter(self) -> impl Iterator<Item = T> {
            self.0.into_iter()
        }

        /// Map each element to `f(element)`.
        #[inline]
        pub fn map<U, F: Fn(T) -> U>(self, f: F) -> Vector<U, N> {
            Vector(self.0.map(f))
        }
    }

    impl<T, const N: usize> Vector<T, N>
    where
        (): SupportedSize<N>,
    {
        /// Is `predicate` true for all elements?.
        /// ```
        /// # use vector::*;
        /// assert!(vec2(1, 2).all(|v| v > 0));
        /// ```
        #[inline]
        pub fn all<F: Fn(T) -> bool>(self, predicate: F) -> bool {
            self.map(predicate).reduce(|a, b| a && b)
        }

        /// Is `predicate` true for any of the elements?.
        /// ```
        /// # use vector::*;
        /// assert!(vec2(0, 1).any(|v| v == 0));
        /// ```
        #[inline]
        pub fn any<F: Fn(T) -> bool>(self, f: F) -> bool {
            self.map(f).reduce(|a, b| a || b)
        }

        /// Reduce elements via `f` (assumed associative).
        /// ```
        /// # use vector::*;
        /// # use std::ops::Add;
        /// assert_eq!(vec3(1, 2, 3).reduce(i32::add), 6);
        /// ```
        #[inline]
        pub fn reduce<F: Fn(T, T) -> T>(self, f: F) -> T {
            <()>::reduce(self, f)
        }

        /// Zip `vec<T>`, `vec<U>` into `vec<(T, U)>`.
        #[inline]
        pub fn zip<U>(self, rhs: Vector<U, N>) -> Vector<(T, U), N> {
            <()>::zip(self, rhs)
        }

        /// Zip and apply `f` to each resulting pair.
        /// ```
        /// # use vector::*;
        /// # use std::ops::Add;
        /// assert_eq!(vec2(1, 2).zip_with(vec2(3, 4), i32::add), vec2(4, 6));
        /// ```
        #[inline]
        pub fn zip_with<U, V, F: Fn(T, U) -> V>(self, rhs: Vector<U, N>, f: F) -> Vector<V, N> {
            <()>::zip_with(self, rhs, f)
        }
    }

    impl<T, const N: usize> Vector<T, N>
    where
        (): SupportedSize<N>,
        T: Add<Output = T>,
    {
        /// Sum of elements.
        #[inline]
        pub fn sum(self) -> T {
            self.reduce(T::add)
        }
    }

    impl<T, const N: usize> Vector<T, N>
    where
        (): SupportedSize<N>,
        T: Mul<Output = T>,
    {
        /// Product of elements.
        #[inline]
        pub fn product(self) -> T {
            self.reduce(T::mul)
        }
    }
}

/// The indexing operator `[]`.
/// ```
/// # use vector::*;
/// assert_eq!(vec3(1,2,3)[0], 1);
/// assert_eq!(vec3(1,2,3)[1], 2);
/// ```
impl<T, const N: usize> Index<usize> for Vector<T, N> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

/// The indexing operator `[]`.
/// ```
/// # use vector::*;
/// let mut v = vec2(0, 0);
/// v[1] = 42;
/// assert_eq!(v, vec2(0, 42))
/// ```
impl<T, const N: usize> IndexMut<usize> for Vector<T, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

/// The addition operator `+`.
/// ```
/// # use vector::*;
/// assert_eq!(vec2(1,2) + vec2(3,4), vec2(4,6));
/// assert_eq!(vec3u(1,2,3) + vec3u(4,5,6), vec3u(5,7,9));
/// ```
impl<T, const N: usize> Add for Vector<T, N>
where
    (): SupportedSize<N>,
    T: Add<Output = T>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        <()>::zip_with(self, rhs, T::add)
    }
}

/// Vector + constant adds the constant to each component.
/// ```
/// # use vector::*;
/// assert_eq!(vec2(1,2) + 3, vec2(4,5));
/// assert_eq!(vec3u(1,2,3) + 1, vec3u(2,3,4));
/// ```
impl<T, const N: usize> Add<T> for Vector<T, N>
where
    (): SupportedSize<N>,
    T: Add<Output = T> + Copy,
{
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        self + Vector::splat(rhs)
    }
}

/// The addition assignment operator `+=`.
/// ```
/// # use vector::*;
/// let mut v = vec2(3, 4);
/// v += vec2(1, 2);
/// assert_eq!(v, vec2(4,6));
/// ```
impl<T, const N: usize> AddAssign<Vector<T, N>> for Vector<T, N>
where
    (): SupportedSize<N>,
    T: AddAssign,
{
    #[inline(always)]
    fn add_assign(&mut self, rhs: Self) {
        <()>::zip_mut_with(self, rhs, T::add_assign)
    }
}

/// The addition assignment operator `+=` with scalar argument.
/// Adds `rhs` to each vector component.
/// ```
/// # use vector::*;
/// let mut v = vec2(3, 4);
/// v += 5;
/// assert_eq!(v, vec2(8, 9));
/// ```
impl<T, const N: usize> AddAssign<T> for Vector<T, N>
where
    (): SupportedSize<N>,
    T: AddAssign + Copy,
{
    #[inline(always)]
    fn add_assign(&mut self, rhs: T) {
        self.add_assign(Self::splat(rhs));
    }
}

/// The addition assignment operator `+=`.
/// ```
/// # use vector::*;
/// let mut v = vec2(3, 4);
/// v += (1, 2);
/// assert_eq!(v, vec2(4,6));
/// ```
impl<T> AddAssign<(T, T)> for Vector<T, 2>
where
    T: AddAssign,
{
    #[inline(always)]
    fn add_assign(&mut self, rhs: (T, T)) {
        <()>::zip_mut_with(self, rhs.into(), T::add_assign)
    }
}

/// The subtraction assignment operator `-=` with scalar argument.
/// Adds `rhs` to each vector component.
/// ```
/// # use vector::*;
/// let mut v = vec2(3, 4);
/// v -= 1;
/// assert_eq!(v, vec2(2, 3));
/// ```
impl<T, const N: usize> SubAssign<T> for Vector<T, N>
where
    (): SupportedSize<N>,
    T: SubAssign + Copy,
{
    #[inline(always)]
    fn sub_assign(&mut self, rhs: T) {
        self.sub_assign(Self::splat(rhs));
    }
}

/// The division operator `/`.
/// ```
/// # use vector::*;
/// assert_eq!(vec2(10.0, 20.0) / vec2(2.0, 5.0), vec2(5.0, 4.0));
/// ```
impl<T, const N: usize> Div for Vector<T, N>
where
    (): SupportedSize<N>,
    T: Div<Output = T>,
{
    type Output = Vector<T, N>;

    #[inline(always)]
    fn div(self, rhs: Self) -> Self::Output {
        <()>::zip_with(self, rhs, T::div)
    }
}

/// The division operator `/`.
/// ```
/// # use vector::*;
/// assert_eq!(vec2(10.0, 20.0) / 2.0, vec2(5.0, 10.0));
/// ```
impl<T, const N: usize> Div<T> for Vector<T, N>
where
    (): SupportedSize<N>,
    T: Div<Output = T> + Copy,
{
    type Output = Vector<T, N>;

    #[inline(always)]
    fn div(self, rhs: T) -> Self::Output {
        self.map(|v| v / rhs)
    }
}

/// The division assignment operator `/=`.
/// ```
/// # use vector::*;
/// let mut v = vec2(30, 50);
/// v /= vec2(3, 2);
/// assert_eq!(v, vec2(10, 25));
/// ```
impl<T, const N: usize> DivAssign for Vector<T, N>
where
    (): SupportedSize<N>,
    T: DivAssign,
{
    #[inline(always)]
    fn div_assign(&mut self, rhs: Self) {
        <()>::zip_mut_with(self, rhs, T::div_assign)
    }
}

/// The division assignment operator `/=`.
/// ```
/// # use vector::*;
/// let mut v = vec2(30, 50);
/// v /= 2;
/// assert_eq!(v, vec2(15, 25));
/// ```
impl<T, const N: usize> DivAssign<T> for Vector<T, N>
where
    (): SupportedSize<N>,
    T: DivAssign + Copy,
{
    #[inline(always)]
    fn div_assign(&mut self, rhs: T) {
        <()>::for_each(self, |v| *v /= rhs)
    }
}

/// The multiplication operator `*`.
/// ```
/// # use vector::*;
/// assert_eq!(vec2(1,2) * vec2(3,4), vec2(3,8));
/// assert_eq!(vec3u(1,2,3) * vec3u(4,5,6), vec3u(4,10,18));
/// ```
impl<T, const N: usize> Mul<Self> for Vector<T, N>
where
    (): SupportedSize<N>,
    T: Mul<Output = T>,
{
    type Output = Vector<T, N>;

    #[inline(always)]
    fn mul(self, rhs: Self) -> Self::Output {
        <()>::zip_with(self, rhs, T::mul)
    }
}

/// The multiplication operator `*`.
/// ```
/// # use vector::*;
/// assert_eq!(vec2(1,2) * 2, vec2(2,4));
/// assert_eq!(vec3u(1,2,3) * 4, vec3u(4,8,12));
/// ```
impl<T, const N: usize> Mul<T> for Vector<T, N>
where
    (): SupportedSize<N>,
    T: Mul<Output = T> + Copy,
{
    type Output = Vector<T, N>;

    #[inline(always)]
    fn mul(self, rhs: T) -> Self::Output {
        self.map(|v| v * rhs)
    }
}

/// Left-multiply primitive types (f32,...) with `vec`. E.g. `2 * vec3(1, 2, 3)`. 
#[rustfmt::skip]
mod mul_primitive{
	use super::*;

/// The multiplication operator `*`.
/// ```
/// # use vector::*;
/// assert_eq!(2.0 * vec2(1.0,2.0), vec2(2.0, 4.0));
/// ```
impl<const N: usize> Mul<Vector<f64, N>> for f64 { type Output = Vector<f64, N>; #[inline(always)] fn mul(self, rhs: Vector<f64, N>) -> Self::Output { rhs.map(|v| self * v) } }
impl<const N: usize> Mul<Vector<f32, N>> for f32 { type Output = Vector<f32, N>; #[inline(always)] fn mul(self, rhs: Vector<f32, N>) -> Self::Output { rhs.map(|v| self * v) } }
impl<const N: usize> Mul<Vector<u64, N>> for u64 { type Output = Vector<u64, N>; #[inline(always)] fn mul(self, rhs: Vector<u64, N>) -> Self::Output { rhs.map(|v| self * v) } }
impl<const N: usize> Mul<Vector<u32, N>> for u32 { type Output = Vector<u32, N>; #[inline(always)] fn mul(self, rhs: Vector<u32, N>) -> Self::Output { rhs.map(|v| self * v) } }
impl<const N: usize> Mul<Vector<u16, N>> for u16 { type Output = Vector<u16, N>; #[inline(always)] fn mul(self, rhs: Vector<u16, N>) -> Self::Output { rhs.map(|v| self * v) } }
impl<const N: usize> Mul<Vector<u8 , N>> for u8  { type Output = Vector<u8 , N>; #[inline(always)] fn mul(self, rhs: Vector<u8 , N>) -> Self::Output { rhs.map(|v| self * v) } }
impl<const N: usize> Mul<Vector<i64, N>> for i64 { type Output = Vector<i64, N>; #[inline(always)] fn mul(self, rhs: Vector<i64, N>) -> Self::Output { rhs.map(|v| self * v) } }
impl<const N: usize> Mul<Vector<i32, N>> for i32 { type Output = Vector<i32, N>; #[inline(always)] fn mul(self, rhs: Vector<i32, N>) -> Self::Output { rhs.map(|v| self * v) } }
impl<const N: usize> Mul<Vector<i16, N>> for i16 { type Output = Vector<i16, N>; #[inline(always)] fn mul(self, rhs: Vector<i16, N>) -> Self::Output { rhs.map(|v| self * v) } }
impl<const N: usize> Mul<Vector<i8 , N>> for i8  { type Output = Vector<i8 , N>; #[inline(always)] fn mul(self, rhs: Vector<i8 , N>) -> Self::Output { rhs.map(|v| self * v) } }
}

/// The multiplication assignment operator `*=`.
/// ```
/// # use vector::*;
/// let mut v = vec2(3, 4);
/// v *= vec2(1, 2);
/// assert_eq!(v, vec2(3,8));
/// ```
impl<T, const N: usize> MulAssign for Vector<T, N>
where
    (): SupportedSize<N>,
    T: MulAssign,
{
    #[inline(always)]
    fn mul_assign(&mut self, rhs: Self) {
        <()>::zip_mut_with(self, rhs, T::mul_assign)
    }
}

/// The multiplication assignment operator `*=`.
/// ```
/// # use vector::*;
/// let mut v = vec2(3, 4);
/// v *= 3;
/// assert_eq!(v, vec2(9, 12));
/// ```
impl<T, const N: usize> MulAssign<T> for Vector<T, N>
where
    (): SupportedSize<N>,
    T: MulAssign + Copy,
{
    #[inline(always)]
    fn mul_assign(&mut self, rhs: T) {
        <()>::for_each(self, |v| *v *= rhs)
    }
}

/// The unary negation operator `-`.
/// ```
/// # use vector::*;
/// assert_eq!(-vec2i(1, 2), vec2i(-1, -2));
/// ```
impl<T, const N: usize> Neg for Vector<T, N>
where
    (): SupportedSize<N>,
    T: Neg<Output = T>,
{
    type Output = Vector<T, N>;

    #[inline(always)]
    fn neg(self) -> Self::Output {
        self.map(T::neg)
    }
}

/// The subtraction operator `-`.
/// ```
/// # use vector::*;
/// assert_eq!(vec2(10,20) - vec2(1,2), vec2(9,18));
/// assert_eq!(vec3u(4,5,6) - vec3u(1,2,3), vec3u(3,3,3));
/// ```
impl<T, const N: usize> Sub for Vector<T, N>
where
    (): SupportedSize<N>,
    T: Sub<Output = T>,
{
    type Output = Vector<T, N>;

    #[inline(always)]
    fn sub(self, rhs: Self) -> Self::Output {
        <()>::zip_with(self, rhs, T::sub)
    }
}

/// Vector - constant subtracts the constant from each component.
/// ```
/// # use vector::*;
/// assert_eq!(vec2(4,5) - 3, vec2(1,2));
/// assert_eq!(vec3u(1,2,3) - 1, vec3u(0,1,2));
/// ```
impl<T, const N: usize> Sub<T> for Vector<T, N>
where
    (): SupportedSize<N>,
    T: Sub<Output = T> + Copy,
{
    type Output = Self;

    fn sub(self, rhs: T) -> Self::Output {
        self - Vector::splat(rhs)
    }
}

/// The addition assignment operator `+=`.
/// ```
/// # use vector::*;
/// let mut v = vec2(3, 5);
/// v -= vec2(1, 2);
/// assert_eq!(v, vec2(2, 3));
/// ```
impl<T, const N: usize> SubAssign for Vector<T, N>
where
    (): SupportedSize<N>,
    T: SubAssign,
{
    #[inline(always)]
    fn sub_assign(&mut self, rhs: Self) {
        <()>::zip_mut_with(self, rhs, T::sub_assign)
    }
}

impl<T, const N: usize> Vector<T, N>
where
    (): SupportedSize<N>,
    T: Add<Output = T> + Mul<Output = T> + Copy,
{
    /// The dot (inner) product of two vectors.
    /// ```
    /// # use vector::*;
    /// assert_eq!(vec2(2, 3).dot(vec2(4, 5)), 23);
    /// assert_eq!(vec3(1, 2, 3).dot(vec3(0, 0, 4)), 12);
    /// assert_eq!(vec4(1, 2, 3, 4).dot(vec4(0, 1, 0, 0)), 2);
    /// ```
    #[inline]
    pub fn dot(self, rhs: Self) -> T {
        (self * rhs).sum()
    }
}

// Too prone to overflow
// impl<T, const N: usize> Vector<T, N>
// where
//     (): SupportedSize<N>,
//     T: Add<Output = T> + Mul<Output = T> + Sub<Output = T> + Copy,
// {
//     /// Distance between two points, squared
//     /// ```
//     /// # use vector::*;
//     /// assert_eq!(vec2i(3, 4).distance_squared(vec2i(2, 4)), 1);
//     /// ```
//     #[inline]
//     pub fn distance_squared(self, rhs: Self) -> T {
//         (self - rhs).len2()
//     }
// }

impl<const N: usize> Vector<i32, N>
where
    (): SupportedSize<N>,
{
    /// Distance between two points, squared
    /// ```
    /// # use vector::*;
    /// assert_eq!(vec2i(3, 4).distance_squared(vec2i(2, 4)), 1);
    /// ```
    #[inline]
    pub fn distance_squared(self, rhs: Self) -> i64 {
        (self.as_i64() - rhs.as_i64()).len2()
    }
}

impl<const N: usize> Vector<i16, N>
where
    (): SupportedSize<N>,
{
    /// Distance between two points, squared ```
    /// # use vector::*;
    /// assert_eq!(vec2(3i64, 4).distance_squared(vec2(2i64, 4)), 1);
    /// ```
    #[inline]
    pub fn distance_squared(self, rhs: Self) -> i32 {
        let diff = self.as_i32() - rhs.as_i32();
        diff.dot(diff)
    }
}

impl<const N: usize> Vector<i64, N>
where
    (): SupportedSize<N>,
{
    /// Length squared.
    /// Prone to overflow, so not provided on smaller types (i32, i16, etc).
    #[inline]
    pub fn len2(self) -> i64 {
        self.as_i64().dot(self.as_i64())
    }
}

impl<T, const N: usize> Vector<T, N>
where
    (): SupportedSize<N>,
    T: Float,
{
    /// Length (norm) of a vector.
    /// ```
    /// # use vector::*;
    /// assert_eq!(vec2f(3.0, 4.0).len(), 5.0);
    /// assert_eq!(vec3d(0.0, 3.0, 4.0).len(), 5.0);
    /// ```
    #[inline]
    pub fn len(self) -> T {
        self.dot(self).sqrt()
    }

    /// Distance between two points.
    /// ```
    /// # use vector::*;
    /// assert_eq!(vec2f(3.0, 4.0).distance_to(vec2f(2.0, 4.0)), 1.0);
    /// ```
    #[inline]
    pub fn distance_to(self, rhs: Self) -> T {
        (self - rhs).len()
    }

    /// Vector with same direction but length normalized to 1,
    /// unless length was zero.
    /// ```
    /// # use vector::*;
    /// assert_eq!(vec3(0.0, 3.0, 4.0).normalized(), vec3(0.0, 3.0, 4.0) / 5.0);
    /// assert_eq!(vec2(0.0, 0.0).normalized(), vec2(0.0, 0.0));
    /// ```
    #[must_use]
    #[inline]
    pub fn normalized(self) -> Self {
        let len = self.len();
        if len == T::zero() { self } else { self / len }
    }

    /// Like `normalized`, but in-place.
    #[inline]
    pub fn normalize(&mut self) {
        *self = self.normalized()
    }

    #[inline]
    pub fn is_finite(self) -> bool {
        self.all(|v| v.is_finite())
    }
}

impl<const N: usize> Vector<f64, N>
where
    (): SupportedSize<N>,
{
    /// Length squared.
    /// Prone to overflow, so not provided on all types (i32, i16, etc).
    #[inline]
    pub fn len2(self) -> f64 {
        self.dot(self)
    }
}

impl<const N: usize> Vector<f32, N>
where
    (): SupportedSize<N>,
{
    /// Length squared.
    /// Prone to overflow, so not provided on all types (i32, i16, etc).
    #[inline]
    pub fn len2(self) -> f32 {
        self.dot(self)
    }
}

impl<T> vec3<T>
where
    T: Copy + Mul<T, Output = T> + Sub<T, Output = T>,
{
    /// Cross product.
    /// ```
    /// # use vector::*;
    /// assert_eq!(vec3(1,0,0).cross(vec3(0,1,0)), vec3(0,0,1));
    /// ```
    #[inline]
    pub fn cross(self, rhs: Self) -> Self {
        Self([self.y() * rhs.z() - self.z() * rhs.y(), self.z() * rhs.x() - self.x() * rhs.z(), self.x() * rhs.y() - self.y() * rhs.x()])
    }
}

impl<T> vec2<T>
where
    T: Copy + Mul<T, Output = T> + Sub<T, Output = T> + Number,
{
    /// Cross product.
    /// ```
    /// # use vector::*;
    /// assert_eq!(vec2(1,0).cross(vec2(0,1)), 1);
    /// ```
    #[inline]
    pub fn cross(self, rhs: Self) -> T {
        self.append(T::ZERO).cross(rhs.append(T::ZERO)).z()
    }
}

impl<const N: usize> Vector<f32, N>
where
    (): SupportedSize<N>,
{
    #[inline]
    pub fn floor(self) -> Vector<i32, N> {
        self.map(|v| v.floor() as i32)
    }

    #[inline]
    pub fn round(self) -> Vector<i32, N> {
        self.map(|v| v.round() as i32)
    }
}

impl<T, const N: usize> Vector<T, N>
where
    T: PartialOrd,
{
    /// Index of the largest element.
    /// ```
    /// # use vector::*;
    /// assert_eq!(vec3(1,0,0).argmax(), 0);
    /// assert_eq!(vec3(0,1,0).argmax(), 1);
    /// assert_eq!(vec3(0,0,1).argmax(), 2);
    /// assert_eq!(vec4(0,0,0,1).argmax(), 3);
    /// ```
    pub fn argmax(&self) -> usize {
        let mut arg = 0;
        for i in 1..N {
            if self[i] > self[arg] {
                arg = i
            }
        }
        arg
    }
}

impl<T, const N: usize> std::iter::Sum for Vector<T, N>
where
    Vector<T, N>: Add<Output = Vector<T, N>> + Default,
{
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|a, b| a + b).unwrap_or_default()
    }
}

// As of 2024, serde does not support [T;N] for arbitrary N (https://github.com/serde-rs/serde/issues/1937).
// When supported, these impls can be replaced by #[derive].
impl<T, const N: usize> Serialize for Vector<T, N>
where
    [T; N]: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl<'de, T, const N: usize> Deserialize<'de> for Vector<T, N>
where
    [T; N]: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Self(Deserialize::deserialize(deserializer)?))
    }
}

unsafe impl<T, const N: usize> Zeroable for Vector<T, N> where T: Pod + Zeroable {}
unsafe impl<T, const N: usize> Pod for Vector<T, N> where T: Pod + Zeroable {}
