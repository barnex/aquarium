use num_traits::AsPrimitive;
use std::ops::{Add, Mul};
use vector::*;

/// 2D Barycentric coordinates of a point with respect to a triangle with given vertices.
/// See <https://en.wikipedia.org/wiki/Barycentric_coordinate_system>.
/// ```
/// # use geometry::*;
/// # use vector::*;
/// //  c(1,3)*
/// //        | \
/// //        |  \
/// //  a(1,1)*---* b(2,1)
/// let a = vec2(1.0, 1.0);
/// let b = vec2(2.0, 1.0);
/// let c = vec2(1.0, 3.0);
/// let triangle = [a, b, c];
/// assert_eq!(barycentric_coordinates(&triangle, a), vec3(1.0, 0.0, 0.0));
/// assert_eq!(barycentric_coordinates(&triangle, b), vec3(0.0, 1.0, 0.0));
/// assert_eq!(barycentric_coordinates(&triangle, c), vec3(0.0, 0.0, 1.0));
/// assert_eq!(barycentric_coordinates(&triangle, (a+b)/2.0), vec3(0.5, 0.5, 0.0));
/// ```
pub fn barycentric_coordinates(triangle_vertices: &[vec2f; 3], point: vec2f) -> vec3f {
    let [(x1, y1), (x2, y2), (x3, y3)] = triangle_vertices.map(|v| v.tuple());
    let (x, y) = point.tuple();
    let det = (y2 - y3) * (x1 - x3) + (x3 - x2) * (y1 - y3);
    let lambda1 = ((y2 - y3) * (x - x3) + (x3 - x2) * (y - y3)) / det;
    let lambda2 = ((y3 - y1) * (x - x3) + (x1 - x3) * (y - y3)) / det;
    let lambda3 = 1.0 - lambda1 - lambda2;
    vec3f(lambda1, lambda2, lambda3)
}

/// Does a 2D point lie inside a triangle defined by the given vertices?
/// ```
/// # use geometry::*;
/// # use vector::*;
/// //  c(1,3)*
/// //        | \
/// //        |  \
/// //  a(1,1)*---* b(2,1)
/// let a = vec2(1.0, 1.0);
/// let b = vec2(2.0, 1.0);
/// let c = vec2(1.0, 3.0);
/// let triangle = [a, b, c];
/// assert_eq!(is_inside(&triangle, a), true);
/// assert_eq!(is_inside(&triangle, b), true);
/// assert_eq!(is_inside(&triangle, c), true);
/// assert_eq!(is_inside(&triangle, vec2(1.5, 2.0)), true);
/// assert_eq!(is_inside(&triangle, vec2(0.9, 2.0)), false);
/// assert_eq!(is_inside(&triangle, vec2(1.5, 2.5)), false);
/// ```
pub fn is_inside(triangle_vertices: &[vec2f; 3], point: vec2f) -> bool {
    barycentric_coordinates(triangle_vertices, point).all(|v| (0.0..=1.0).contains(&v))
}

/// Interpolate between 3 vertex values, using the given scalar weights.
/// Weights are typically barycentric coordinates.
///
/// Weights and values often have different types.
/// E.g. interpolate vertex colors inside a triangle.
///
#[inline]
pub fn barycentric_interpolation<IN, WEIGHT, OUT>(v: vec3<IN>, weight: vec3<WEIGHT>) -> OUT
where
    IN: Mul<WEIGHT, Output = OUT> + Copy,
    WEIGHT: Copy + AsPrimitive<f64>, // 👈 `AsPrimitive` to make sure that weights are scalars (rot strictly needed).
    OUT: Add<Output = OUT> + Copy,
{
    v[0] * weight[0] + v[1] * weight[1] + v[2] * weight[2]
}
