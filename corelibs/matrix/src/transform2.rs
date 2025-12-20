use super::*;
use vector::*;

impl mat2x2f {
    /// Counterclockwise rotation (if X points right and Y up).
    ///
    ///```text
    ///    y
    ///    ^  p'
    ///    | /
    ///    |/⤺θ
    ///    +---p--> x
    ///```
    ///
    pub fn ccw_rotation(angle: f32) -> Self {
        let (sin, cos) = f32::sin_cos(angle);
        Self::from([[cos, sin], [-sin, cos]])
    }
}

impl mat4x4f {
    /// Matrix for rotation around an arbitrary axis
    /// <https://en.wikipedia.org/wiki/Rotation_matrix#Rotation_matrix_from_axis_and_angle>.
    pub fn rotation_matrix(axis: vec3f, radians: f32) -> mat4x4f {
        let axis = axis.normalized();
        let (ux, uy, uz) = (axis.x(), axis.y(), axis.z());
        let c = f32::cos(radians);
        let s = f32::sin(radians);
        let c1 = 1.0 - c;
        mat4x4::from([
            [c + ux * ux * c1, uy * ux * c1 + uz * s, uz * ux * c1 - uy * s, 0.0],
            [ux * uy * c1 - uz * s, c + uy * uy * c1, uz * uy * c1 + ux * s, 0.0],
            [ux * uz * c1 + uy * s, uy * uz * c1 - ux * s, c + uz * uz * c1, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    // TODO: check handedness OpenGL vs. WGPU
    pub fn yaw_matrix(yaw: f32) -> mat4x4f {
        let (sin, cos) = f32::sin_cos(yaw);
        mat4x4f::new_transposed([
            [cos, 0.0, -sin, 0.0], //
            [0.0, 1.0, 0.0, 0.0],
            [sin, 0.0, cos, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn pitch(pitch: f32) -> mat4x4f {
        let (sin, cos) = f32::sin_cos(pitch);
        //let sin = f32::sin(pitch);
        //let cos = f32::cos(pitch);
        mat4x4f::new_transposed([
            [1.0, 0.0, 0.0, 0.0], //
            [0.0, cos, -sin, 0.0],
            [0.0, sin, cos, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    // A rotation matrix that yaws (rotate around Y), then pitches (rotate around X).
    pub fn yaw_pitch_matrix(yaw: f32, pitch: f32) -> mat4x4f {
        Self::pitch(pitch) * Self::yaw_matrix(yaw)
    }

    pub fn translation(delta: vec3f) -> mat4x4f {
        mat4x4f::from([
            [1.0, 0.0, 0.0, 0.0], //
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [delta.x(), delta.y(), delta.z(), 1.0],
        ])
    }

    pub fn scale_matrix(scl: f32) -> mat4x4f {
        mat4x4f::from([
            [scl, 0.0, 0.0, 0.0], //
            [0.0, scl, 0.0, 0.0],
            [0.0, 0.0, scl, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn scale_anisotropic(s: vec3f) -> mat4x4f {
        mat4x4f::from([
            [s.x(), 0.0, 0.0, 0.0], //
            [0.0, s.y(), 0.0, 0.0],
            [0.0, 0.0, s.z(), 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }
}
