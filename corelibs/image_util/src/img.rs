use crate::*;
use core_util::*;
use geometry::*;
use serde::{Deserialize, Serialize};
use std::iter;
use std::ops::{Add, AddAssign, Index, IndexMut};
use vector::*;

/// An Image is a rectangular 2D array of color values
/// (RGB, grayscale, ...)
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Img<C> {
	size: vec2u,
	values: Vec<C>,
}

impl<C> Img<C> {
	pub fn from_fn<F>(size: vec2u, mut f: F) -> Self
	where
		F: FnMut(vec2u) -> C,
	{
		let mut values = Vec::with_capacity((size.x() * size.y()) as usize);
		let (w, h) = size.into();
		for iy in 0..h {
			for ix in 0..w {
				values.push(f(vec2u(ix, iy)));
			}
		}
		Self { size, values }
	}
}

impl<C> Img<C>
where
	C: Default,
{
	/// new constructs an image with given width and height and filled with the default value.
	pub fn new(size: vec2u) -> Img<C> {
		Self::from_fn(size, |_| C::default())
	}
}

impl<C> Img<C> {
	/// width of the image, in pixels
	#[inline]
	pub fn width(&self) -> u32 {
		self.size.x()
	}

	/// height of the image, in pixels
	#[inline]
	pub fn height(&self) -> u32 {
		self.size.y()
	}

	/// width and height of the image
	#[inline]
	pub fn size(&self) -> vec2u {
		self.size
	}

	/// pixels in row-major order, iterable.
	#[inline]
	pub fn pixels(&self) -> &[C] {
		&self.values
	}

	/// pixels in row-major order, iterable.
	#[inline]
	pub fn pixels_mut(&mut self) -> &mut [C] {
		&mut self.values
	}

	pub fn pixel_positions(&self) -> impl Iterator<Item = vec2u> + 'static{
		cross(0..self.width(), 0..self.height()).map(vec2u::from)
	}

	pub fn enumerate(&self) -> impl Iterator<Item = (vec2u, &C)> {
		cross(0..self.width(), 0..self.height()).map(vec2u::from).map(|i| (i, self.ref_at(i)))
	}

	pub fn map<F, T>(&self, f: F) -> Img<T>
	where
		F: Fn(&C) -> T,
	{
		Img::<T> {
			size: self.size,
			values: self.values.iter().map(f).collect(),
		}
	}

	#[inline]
	#[track_caller]
	pub fn mut_at<P: Into<vec2u>>(&mut self, p: P) -> &mut C {
		let p: vec2u = p.into();
		&mut self[p.y() as usize][p.x() as usize]
	}

	#[inline]
	#[track_caller]
	pub fn ref_at<P: Into<vec2u>>(&self, p: P) -> &C {
		let p: vec2u = p.into();
		&self[p.y() as usize][p.x() as usize]
	}

	#[inline]
	#[track_caller]
	pub fn get(&self, p: vec2u) -> Option<&C> {
		(p.x() < self.width() && p.y() < self.height()).then(|| self.ref_at(p))
	}

	#[inline]
	#[track_caller]
	pub fn set(&mut self, p: impl Into<vec2u>, c: C) {
		let p: vec2u = p.into();
		self[p.y() as usize][p.x() as usize] = c;
	}

	#[inline]
	#[track_caller]
	pub fn get_mut(&mut self, p: vec2u) -> Option<&mut C> {
		(p.x() < self.width() && p.y() < self.height()).then(|| self.mut_at(p))
	}

	#[inline]
	#[track_caller]
	pub fn try_set_i32(&mut self, idx: vec2i, v: C) {
		if self.in_bounds(idx) {
			self.set(idx.as_u32(), v);
		}
	}

	#[inline]
	#[track_caller]
	pub fn in_bounds(&self, idx: vec2i) -> bool {
		idx.all(|v| v >= 0) //_
			&& (idx.x() as u32) < self.width() //_
			&& (idx.y() as u32) < self.height()
	}
}

impl<C> Img<C> {
	/// Convert to 8-bit SRGB using the given (linear) tone mapping function.
	pub fn map_to_srgb(&self, f: impl Fn(&C) -> vec3f) -> image::RgbImage {
		image::RgbImage::from_fn(
			self.width(),
			self.height(), //
			|x, y| image::Rgb(f(self.ref_at((x, y))).map(linear_to_srgb).into()),
		)
	}
}

impl Img<vec3<u8>> {
	pub fn to_rgb_image(&self) -> image::RgbImage {
		image::RgbImage::from_fn(
			self.width(),
			self.height(), //
			|x, y| image::Rgb(self.at((x, y)).into()),
		)
	}
}

impl<C> Img<C>
where
	C: Copy,
{
	/// Draw img over this image.
	pub fn draw(&mut self, pos: vec2u, src: &Img<C>) {
		for y in 0..src.height() {
			for x in 0..src.width() {
				let dst = (pos.x() + x, pos.y() + y);
				if dst.0 < self.width() && dst.1 < self.height() {
					self.set(dst, src.at((x, y)));
				}
			}
		}
	}

	pub fn draw_flipped(&mut self, pos: vec2u, src: &Img<C>) {
		for y in 0..src.height() {
			for x in 0..src.width() {
				let dst = (pos.x() + x, pos.y() + (src.height()-y));
				if dst.0 < self.width() && dst.1 < self.height() {
					self.set(dst, src.at((x, y)));
				}
			}
		}
	}

	pub fn map_values<F, T>(&self, f: F) -> Img<T>
	where
		T: Copy + Default,
		F: Fn(C) -> T,
	{
		Img::<T> {
			size: self.size,
			values: self.values.iter().copied().map(f).collect(),
		}
	}

	pub fn at<P: Into<vec2u>>(&self, p: P) -> C {
		let p: vec2u = p.into();
		self[p.y() as usize][p.x() as usize]
	}

	#[inline]
	pub fn at_i32(&self, p: vec2i) -> Option<C> {
		(self.in_bounds(p)).then(|| self.ref_at(p.as_())).copied()
	}

	/// TODO: separate texture filtering
	pub fn at_uv_nearest_clamp<UV: Into<vec2f>>(&self, uv: UV) -> C {
		let uv = uv.into();
		let pix = linterp(
			//
			vec2f(0.0, 0.0), //
			vec2f(0.0, 0.0), //
			vec2f(1.0, 1.0),
			self.size.as_f32(), //
			uv,                 //
		)
		.map(|v| v.floor() as u32);
		let x = pix.x().clamp(0, self.size().x() - 1);
		let y = pix.y().clamp(0, self.size().y() - 1);
		self.at((x, y))
	}

	pub fn at_uv_nearest_wrap<UV: Into<vec2f>>(&self, uv: UV) -> C {
		let uv = uv.into();

		let wrap = |v| {
			let mut v = v % 1.0;
			while v < 0.0 {
				v += 1.0;
			}
			// TODO: redundant?
			while v >= 1.0 {
				v -= 1.0;
			}
			v
		};

		let uv = uv.map(wrap);

		let pix = linterp(
			//
			vec2f(0.0, 0.0), //
			vec2f(0.0, 0.0), //
			vec2f(1.0, 1.0),
			self.size.as_f32(), //
			uv,                 //
		)
		.map(|v| v.floor() as u32);
		//let x = pix.x().clamp(0, self.size().x() - 1); // should be impossible
		//let y = pix.y().clamp(0, self.size().y() - 1);
		self.at(pix)
	}

	pub fn at_uv_nearest_wrap_mut<UV: Into<vec2f>>(&mut self, uv: UV) -> &mut C {
		let uv = uv.into();

		let wrap = |v| {
			let mut v = v % 1.0;
			while v < 0.0 {
				v += 1.0;
			}
			// TODO: redundant?
			while v >= 1.0 {
				v -= 1.0;
			}
			v
		};

		let uv = uv.map(wrap);

		let pix = linterp(
			//
			vec2f(0.0, 0.0), //
			vec2f(0.0, 0.0), //
			vec2f(1.0, 1.0),
			self.size.as_f32(), //
			uv,                 //
		)
		.map(|v| v.floor() as u32);
		//let x = pix.x().clamp(0, self.size().x() - 1); // should be impossible
		//let y = pix.y().clamp(0, self.size().y() - 1);
		self.at_mut(pix)
	}

	#[inline]
	pub fn try_set(&mut self, p: impl Into<vec2u>, c: C) {
		let p: vec2u = p.into();
		if p.x() < self.width() && p.y() < self.height() {
			self[p.y() as usize][p.x() as usize] = c;
		}
	}
	pub fn fill(&mut self, c: C) {
		self.pixels_mut().iter_mut().for_each(|p| *p = c)
	}
}

impl<C> Img<C> {
	#[inline]
	pub fn try_at_i32(&self, p: vec2i) -> Option<&C> {
		if self.contains(p) {
			Some(&self[p.y() as usize][p.x() as usize])
		} else {
			None
		}
	}

	#[inline]
	pub fn at_mut<P: Into<vec2u>>(&mut self, p: P) -> &mut C {
		let p: vec2u = p.into();
		&mut self[p.y() as usize][p.x() as usize]
	}

	pub fn contains(&self, p: impl Into<vec2i>) -> bool {
		let p = p.into();
		let (x, y) = p.into();
		let w = self.width() as i32;
		let h = self.height() as i32;
		x >= 0 && x < w && y >= 0 && y < h
	}
}

impl<C> Default for Img<C>
where
	C: Copy + Default,
{
	fn default() -> Self {
		Self {
			size: vec2u(0, 0),
			values: Vec::new(),
		}
	}
}

impl<C> Index<usize> for Img<C> {
	type Output = [C];

	fn index(&self, i: usize) -> &[C] {
		let l = i * self.width() as usize;
		let h = l + self.width() as usize;
		&self.values[l..h]
	}
}

impl<C> IndexMut<usize> for Img<C> {
	fn index_mut(&mut self, i: usize) -> &mut [C] {
		let l = i * self.width() as usize;
		let h = l + self.width() as usize;
		&mut self.values[l..h]
	}
}

impl<T> AddAssign for Img<T>
where
	T: AddAssign + Copy,
{
	fn add_assign(&mut self, rhs: Self) {
		assert!(self.size() == rhs.size());
		iter::zip(self.pixels_mut().iter_mut(), rhs.pixels()).for_each(|(dst, rhs)| *dst += *rhs)
	}
}

impl<T> Add for &Img<T>
where
	T: Add<Output = T> + Copy,
{
	type Output = Img<T>;

	fn add(self, rhs: Self) -> Self::Output {
		assert!(self.size() == rhs.size());
		Img::from_fn(self.size(), |pos| self.at(pos) + rhs.at(pos))
	}
}

impl Img<vec3f> {
	pub fn to_srgb(&self) -> image::RgbImage {
		//RgbImage::from_fn(self.width(), self.height(), |x, y| Rgb(self.at((x, y)).map(linear_to_srgb).into()))
		self.map_to_srgb(|&v| v)
	}

	pub fn from_srgb(src: &image::RgbImage) -> Self {
		let (w, h) = src.dimensions();
		Self::from_fn(vec2u(w, h), |Vector([x, y])| {
			let image::Rgb([r, g, b]) = src.get_pixel(x, y);
			vec3::from([*r, *g, *b]).map(srgb_to_linear)
		})
	}
}
