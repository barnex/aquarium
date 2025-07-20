use crate::*;
use matrix::*;
use serde::{Deserialize, Serialize};
use vector::*;

/// An orientation with a yaw + pitch angle (but no roll).
#[derive(Debug, Default, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Orientation {
	/// Yaw angle with respect to negative Z axis (radians, positive = CCW seen from above).
	pub yaw: f32,
	/// Pitch angle with respect to the horizon (radians, positive = pitch up).
	pub pitch: f32,
}

impl Orientation {
	pub fn from_look_dir(dir: vec3f) -> Self {
		let dir = dir.normalized();
		Self {
			yaw: f32::atan2(dir.x(), dir.z()),
			pitch: -f32::asin(dir.y()),
		}
	}

	/// Unit vector in the looking direction.
	pub fn look_dir(&self) -> vec3f {
		let yaw = self.yaw;
		let pitch = self.pitch;
		let x = f32::sin(yaw) * f32::cos(-pitch);
		let z = f32::cos(yaw) * f32::cos(-pitch);
		let y = f32::sin(-pitch);
		vec3f(x, y, z)
	}

	/// Looking direction, projected on the horizontal plane.
	pub fn look_dir_h(&self) -> vec3f {
		let yaw = self.yaw;
		let x = f32::sin(yaw);
		let z = f32::cos(yaw);
		let y = 0.0;
		vec3f(x, y, z).normalized()
	}

	/// Direction 90 degrees right of look_dir
	pub fn look_right(&self) -> vec3f {
		let look = self.look_dir();
		vec3f(-look.z(), 0.0, look.x()).normalized()
	}

	pub fn look_up(&self) -> vec3f {
		self.look_right().cross(self.look_dir())
	}

	pub fn matrix_neg_z(&self) -> mat4x4f {
		let pitch_mat = pitch_matrix(-self.pitch);
		let yaw_mat = yaw_matrix(180.0 * DEG - self.yaw);
		&yaw_mat * &pitch_mat
	}

	pub fn matrix_pos_z(&self) -> mat4x4f {
		let pitch_mat = pitch_matrix(-self.pitch);
		let yaw_mat = yaw_matrix(self.yaw);
		&yaw_mat * &pitch_mat
	}

	/// Rotate a vector by this orientation's yaw angle,
	/// around the vertical (Y) axis.
	pub fn apply_yaw(&self, rhs: vec3f) -> vec3f {
		let s = -self.yaw.sin();
		let c = -self.yaw.cos();
		let (x, y, z) = rhs.into();

		vec3f(
			x * c + z * s, //
			y,
			-x * s + z * c,
		)
	}

	//#[must_use]
	//pub fn add(&self, rhs: &Self) -> Self {
	//	Self {
	//		yaw: wrap_angle(self.yaw + rhs.yaw),
	//		pitch: wrap_angle(self.pitch + rhs.pitch),
	//	}
	//}
}
