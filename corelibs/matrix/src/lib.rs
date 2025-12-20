#![allow(non_camel_case_types)]
#![deny(warnings)]

//! 2, 3 and 4-component matrix types similar to [WGSL](https://www.w3.org/TR/WGSL/#matrix).

/// Linear transformations (rotate, scale, ...).
mod transform2;
mod transforms;
pub use transforms::*;

//#[cfg(feature = "inspect")]
//mod inspect;

use bytemuck::{Pod, Zeroable};
use serde::{Deserialize, Serialize};
use std::ops::{Add, Index, IndexMut, Mul};
use vector::Vector;

/// Generic `N`x`N` matrix. Base type for `mat2x2`, `mat3x3`, `mat4x4`.
/// Column-major indexing (like WGSL/OpenGL).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct Matrix<T, const N: usize>(pub [[T; N]; N]);

impl<T, const N: usize> Default for Matrix<T, N>
where
    [[T; N]; N]: Default,
{
    fn default() -> Self {
        Self(<[[T; N]; N]>::default())
    }
}

/// 2 x 2 matrix.
pub type mat2x2<T> = Matrix<T, 2>;
/// 3 x 3 matrix.
pub type mat3x3<T> = Matrix<T, 3>;
/// 4 x 4 matrix.
pub type mat4x4<T> = Matrix<T, 4>;

/// 2 x 2 matrix of f32, similar to WGSL `mat2x2f`.
pub type mat2x2f = Matrix<f32, 2>;
/// 3 x 3 matrix of f32, similar to WGSL `mat3x3f`.
pub type mat3x3f = Matrix<f32, 3>;
/// 4 x 4 matrix of f32, similar to WGSL `mat4x4f`.
pub type mat4x4f = Matrix<f32, 4>;

impl<T, const N: usize> Index<usize> for Matrix<T, N> {
    type Output = [T; N];

    fn index(&self, index: usize) -> &[T; N] {
        &self.0[index]
    }
}

impl<T, const N: usize> IndexMut<usize> for Matrix<T, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<T, const N: usize> From<[[T; N]; N]> for Matrix<T, N> {
    #[inline(always)]
    fn from(value: [[T; N]; N]) -> Self {
        Self(value)
    }
}

impl<T, const N: usize> From<[Vector<T, N>; N]> for Matrix<T, N> {
    #[inline(always)]
    fn from(value: [Vector<T, N>; N]) -> Self {
        Self(value.map(|v| v.into()))
    }
}

impl<T, const N: usize> From<Matrix<T, N>> for [[T; N]; N] {
    #[inline(always)]
    fn from(value: Matrix<T, N>) -> Self {
        value.0
    }
}

impl<T, const N: usize> Mul<&Matrix<T, N>> for &Matrix<T, N>
where
    Matrix<T, N>: Default,
    T: Mul<Output = T> + Add<Output = T> + Copy,
{
    type Output = Matrix<T, N>;

    /// Matrix-Matrix multiplication.
    /// TODO: check matrix multiplication left vs right + ALL scathanna3 transforms are REVERSED!
    /// ```
    /// # use matrix::*;
    /// # use vector::*;
    /// let a = Matrix([[2, 1], [1, 3]]);
    /// let b = Matrix([[5, 2], [3, 9]]);
    /// let v = vec2(1, 2);
    /// assert_eq!((a*b)*v, a*(b*v)); // wrong
    /// assert_eq!((b*a)*v, b*(a*v));
    ///
    /// assert_eq!(Matrix([[1, 0],[0, 0]]) * vec2(2, 3), vec2(2, 0));
    /// assert_eq!(Matrix([[0, 1],[0, 0]]) * vec2(2, 3), vec2(0, 2));
    /// assert_eq!(Matrix([[0, 0],[1, 0]]) * vec2(2, 3), vec2(3, 0));
    /// assert_eq!(Matrix([[0, 0],[0, 1]]) * vec2(2, 3), vec2(0, 3));
    /// ```
    fn mul(self, rhs: &Matrix<T, N>) -> Self::Output {
        let mut c: Matrix<T, N> = Default::default();
        for i in 0..N {
            for j in 0..N {
                for k in 0..N {
                    c[i][j] = c[i][j] + rhs[i][k] * self[k][j];
                }
            }
        }
        c
    }
}

impl<T, const N: usize> Mul<Vector<T, N>> for &Matrix<T, N>
where
    Vector<T, N>: Default,
    T: Mul<Output = T> + Add<Output = T> + Copy,
{
    type Output = Vector<T, N>;

    /// ```
    /// # use matrix::*;
    /// # use vector::*;
    /// let a = Matrix([[2, 0], [0, 3]]);
    /// assert_eq!(a*vec2(1, 0), vec2(2, 0));
    /// assert_eq!(a*vec2(0, 1), vec2(0, 3));
    /// assert_eq!(a*vec2(1, 1), vec2(2, 3));
    /// assert_eq!(a*vec2(2, 1), vec2(4, 3));
    ///
    /// let b = Matrix([[0, 1, 0], [1, 0, 0], [0, 0, 1]]);
    /// assert_eq!(b*vec3(1, 2, 3), vec3(2, 1, 3));
    ///
    /// let m = Matrix::from([[1.0,2.0,3.0,4.0],[5.0,6.0,7.0,8.0],[9.0,10.0,11.0,12.0],[13.0,14.0,15.0,16.0]]);
    /// assert_eq!(m * (vec4(5.0,6.0,7.0,8.0)), vec4(202.0, 228.0, 254.0, 280.0) );
    /// ```
    fn mul(self, rhs: Vector<T, N>) -> Self::Output {
        let mut c: Vector<T, N> = Default::default();
        for i in 0..N {
            for j in 0..N {
                c[i] = c[i] + self[j][i] * rhs[j];
            }
        }
        c
    }
}

impl<T, const N: usize> Mul<Vector<T, N>> for Matrix<T, N>
where
    Vector<T, N>: Default,
    T: Mul<Output = T> + Add<Output = T> + Copy,
{
    type Output = Vector<T, N>;

    fn mul(self, rhs: Vector<T, N>) -> Self::Output {
        (&self).mul(rhs)
    }
}

// allows chaining multiplications:  &a * &b * &c
impl<T, const N: usize> Mul<&Matrix<T, N>> for Matrix<T, N>
where
    Matrix<T, N>: Default,
    T: Mul<Output = T> + Add<Output = T> + Copy,
{
    type Output = Matrix<T, N>;

    /// Matrix-Matrix multiplication.
    fn mul(self, rhs: &Matrix<T, N>) -> Matrix<T, N> {
        (&self).mul(rhs)
    }
}

impl<T, const N: usize> Mul<Matrix<T, N>> for Matrix<T, N>
where
    Matrix<T, N>: Default,
    T: Mul<Output = T> + Add<Output = T> + Copy,
{
    type Output = Matrix<T, N>;

    /// Matrix-Matrix multiplication.
    fn mul(self, rhs: Matrix<T, N>) -> Matrix<T, N> {
        (&self).mul(&rhs)
    }
}

impl<T> Matrix<T, 4>
where
    T: Copy,
{
    #[inline(always)]
    pub fn new_transposed(v: [[T; 4]; 4]) -> Self {
        Self([[v[0][0], v[1][0], v[2][0], v[3][0]], [v[0][1], v[1][1], v[2][1], v[3][1]], [v[0][2], v[1][2], v[2][2], v[3][2]], [v[0][3], v[1][3], v[2][3], v[3][3]]])
    }

    #[inline(always)]
    pub fn transpose(self) -> Self {
        Self::new_transposed(self.into())
    }
}

impl<T> Matrix<T, 2>
where
    T: vector::Number,
{
    /// 2x2 unit matrix.
    pub const UNIT: Self = Self([
        //_
        [T::ONE, T::ZERO],
        [T::ZERO, T::ONE],
    ]);
}

impl<T> Matrix<T, 3>
where
    T: vector::Number,
{
    /// 3x3 unit matrix.
    pub const UNIT: Self = Self([
        //_
        [T::ONE, T::ZERO, T::ZERO],
        [T::ZERO, T::ONE, T::ZERO],
        [T::ZERO, T::ZERO, T::ONE],
    ]);
}

impl<T> Matrix<T, 4>
where
    T: vector::Number,
{
    /// 4x4 unit matrix.
    pub const UNIT: Self = Self([
        //_
        [T::ONE, T::ZERO, T::ZERO, T::ZERO],
        [T::ZERO, T::ONE, T::ZERO, T::ZERO],
        [T::ZERO, T::ZERO, T::ONE, T::ZERO],
        [T::ZERO, T::ZERO, T::ZERO, T::ONE],
    ]);
}

// As of 2024, serde does not support [T;N] for arbitrary N (https://github.com/serde-rs/serde/issues/1937).
// When supported, these impls can be replaced by #[derive].
impl<T, const N: usize> Serialize for Matrix<T, N>
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

impl<'de, T, const N: usize> Deserialize<'de> for Matrix<T, N>
where
    [[T; N]; N]: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Self(Deserialize::deserialize(deserializer)?))
    }
}

unsafe impl<T, const N: usize> Zeroable for Matrix<T, N> where T: Zeroable {}
unsafe impl<T, const N: usize> Pod for Matrix<T, N> where T: Pod {}
