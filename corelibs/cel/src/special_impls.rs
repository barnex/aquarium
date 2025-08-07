/// Convenience methods on Cel<Vector<T>>.
/// Allows e.g. `cel.x()` instead of `cel.get().x()`.
use crate::*;
use core_util::*;
use vector::*;

// ---------- option

impl<T: Copy> Cel<Option<T>> {
    /// Shorthand for `my_cel.get().map(f)`
    pub fn map<U, F: FnOnce(T) -> U>(&self, f: F) -> Option<U> {
        self.get().map(f)
    }

    /// Shorthand for `my_cel.get().and_then(f)`
    pub fn and_then<U, F: FnOnce(T) -> Option<U>>(&self, f: F) -> Option<U> {
        self.get().and_then(f)
    }

    /// Like `Option::take`: replaces inner value by `None` and return it, if any.
    pub fn take(&self) -> Option<T> {
        let v = self.get();
        self.set(None);
        v
    }

    /// Like `Option::is_none()`.
    pub fn is_none(&self) -> bool {
        self.get().is_none()
    }

    /// Like `Option::is_some()`.
    pub fn is_some(&self) -> bool {
        self.get().is_some()
    }
}

// ---------- vector

impl<T: Copy> Cel<Vector<T, 2>> {
    /// X component.
    #[inline(always)]
    pub fn x(&self) -> T {
        self.get().x()
    }

    /// Y component.
    #[inline(always)]
    pub fn y(&self) -> T {
        self.get().y()
    }

    /// Set X component.
    #[inline(always)]
    pub fn set_x(&self, v: T) {
        self.set(self.get().with(|s| s[0] = v))
    }

    /// Set Y component.
    #[inline(always)]
    pub fn set_y(&self, v: T) {
        self.set(self.get().with(|s| s[1] = v))
    }
}

impl<T: Copy> Cel<Vector<T, 3>> {
    /// X component.
    #[inline(always)]
    pub fn x(&self) -> T {
        self.get().x()
    }

    /// Y component.
    #[inline(always)]
    pub fn y(&self) -> T {
        self.get().y()
    }

    /// Z component.
    #[inline(always)]
    pub fn z(&self) -> T {
        self.get().z()
    }

    /// Set X component.
    #[inline(always)]
    pub fn set_x(&self, v: T) {
        self.set(self.get().with(|s| s[0] = v))
    }

    /// Set Y component.
    #[inline(always)]
    pub fn set_y(&self, v: T) {
        self.set(self.get().with(|s| s[1] = v))
    }

    /// Set Z component.
    #[inline(always)]
    pub fn set_z(&self, v: T) {
        self.set(self.get().with(|s| s[2] = v))
    }
}

impl<T: Copy> Cel<Vector<T, 4>> {
    /// X component.
    #[inline(always)]
    pub fn x(&self) -> T {
        self.get().x()
    }

    /// Y component.
    #[inline(always)]
    pub fn y(&self) -> T {
        self.get().y()
    }

    /// Z component.
    #[inline(always)]
    pub fn z(&self) -> T {
        self.get().z()
    }

    /// W component.
    #[inline(always)]
    pub fn w(&self) -> T {
        self.get().w()
    }

    /// Set X component.
    #[inline(always)]
    pub fn set_x(&self, v: T) {
        self.set(self.get().with(|s| s[0] = v))
    }

    /// Set Y component.
    #[inline(always)]
    pub fn set_y(&self, v: T) {
        self.set(self.get().with(|s| s[1] = v))
    }

    /// Set Z component.
    #[inline(always)]
    pub fn set_z(&self, v: T) {
        self.set(self.get().with(|s| s[2] = v))
    }

    /// Set W component.
    #[inline(always)]
    pub fn set_w(&self, v: T) {
        self.set(self.get().with(|s| s[3] = v))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use googletest::prelude::*;

    #[gtest]
    fn vector_support_2d() {
        let a = vec2(1, 2).cel();
        expect_eq!(a, vec2(1, 2));

        expect_eq!(a.x(), 1);
        expect_eq!(a.y(), 2);

        a.set_x(3);
        expect_eq!(a, vec2(3, 2));

        a.set_y(4);
        expect_eq!(a, vec2(3, 4));
    }

    #[gtest]
    fn vector_support_3d() {
        let a = vec3(1, 2, 3).cel();
        expect_eq!(a, vec3(1, 2, 3));

        expect_eq!(a.x(), 1);
        expect_eq!(a.y(), 2);
        expect_eq!(a.z(), 3);

        a.set_x(4);
        expect_eq!(a, vec3(4, 2, 3));

        a.set_y(5);
        expect_eq!(a, vec3(4, 5, 3));

        a.set_z(6);
        expect_eq!(a, vec3(4, 5, 6));
    }

    #[gtest]
    fn vector_support_4d() {
        let a = vec4(1, 2, 3, 4).cel();
        expect_eq!(a, vec4(1, 2, 3, 4));

        expect_eq!(a.x(), 1);
        expect_eq!(a.y(), 2);
        expect_eq!(a.z(), 3);
        expect_eq!(a.w(), 4);

        a.set_x(5);
        expect_eq!(a, vec4(5, 2, 3, 4));

        a.set_y(6);
        expect_eq!(a, vec4(5, 6, 3, 4));

        a.set_z(7);
        expect_eq!(a, vec4(5, 6, 7, 4));

        a.set_w(8);
        expect_eq!(a, vec4(5, 6, 7, 8));
    }
}
