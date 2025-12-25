use crate::*;

use std::cell::Cell;
use std::fmt::{Debug, Display};
use std::ops::{Add, Sub};

#[derive(Default)]
pub struct Cel<T: Copy>(Cell<T>);

impl<T: Copy> Cel<T> {
    #[inline(always)]
    pub fn new(v: T) -> Self {
        Self(Cell::new(v))
    }

    #[inline(always)]
    pub fn get(&self) -> T {
        self.0.get()
    }
}

// ---------- set(T), set(&T), set(&Cel<T>)

pub trait Set<T> {
    fn set(&self, v: T);
}

impl<T: Copy> Set<T> for Cel<T> {
    #[inline(always)]
    fn set(&self, v: T) {
        self.0.set(v);
    }
}

impl<T: Copy> Set<&T> for Cel<T> {
    #[inline(always)]
    fn set(&self, v: &T) {
        self.0.set(*v);
    }
}

impl<T: Copy> Set<&Cel<T>> for Cel<T> {
    #[inline(always)]
    fn set(&self, v: &Cel<T>) {
        self.0.set(v.0.get());
    }
}

// ---------- eq

impl<T: Copy + PartialEq> PartialEq<T> for Cel<T> {
    #[inline(always)]
    fn eq(&self, other: &T) -> bool {
        &self.0.get() == other
    }
}

impl<T: Copy + PartialEq> PartialEq<&T> for Cel<T> {
    #[inline(always)]
    fn eq(&self, other: &&T) -> bool {
        &&self.0.get() == other
    }
}

impl<T> PartialEq<Cel<T>> for Cel<T>
where
    T: Copy + PartialEq,
{
    #[inline(always)]
    fn eq(&self, other: &Cel<T>) -> bool {
        self.0 == other.0
    }
}

impl<T: Copy + Eq> Eq for Cel<T> {}

// ---------- increment

pub trait CelAdd<T> {
    /// Increment value by `rhs`.
    fn inc(&self, rhs: T);
}

impl<T: Copy + Add<Output = T>> CelAdd<T> for Cel<T> {
    /// Increment value by `rhs`.
    #[inline(always)]
    fn inc(&self, rhs: T) {
        self.set(self.get() + rhs);
    }
}

impl<T: Copy + Add<Output = T>> CelAdd<&T> for Cel<T> {
    /// Increment value by `rhs`.
    #[inline(always)]
    fn inc(&self, rhs: &T) {
        self.set(self.get() + *rhs);
    }
}

impl<T: Copy + Add<Output = T>> CelAdd<&Cel<T>> for Cel<T> {
    /// Increment value by `rhs`.
    #[inline(always)]
    fn inc(&self, rhs: &Cel<T>) {
        self.set(self.get() + rhs.get());
    }
}

impl<T: Copy + Sub<Output = T>> Cel<T> {
    /// Decrement value by `rhs`.
    #[inline(always)]
    pub fn sub(&self, rhs: T) {
        self.set(self.get() - rhs);
    }
}

impl Cel<u8> {
    /// Decrement value by `rhs`.
    #[inline(always)]
    pub fn saturating_sub(&self, rhs: u8) {
        self.set(self.get().saturating_sub(rhs));
    }
}

impl Cel<u16> {
    /// Decrement value by `rhs`.
    #[inline(always)]
    pub fn saturating_sub(&self, rhs: u16) {
        self.set(self.get().saturating_sub(rhs));
    }
}

impl Cel<u8> {
    /// Increment value, but clamped to `max`.
    /// E.g.:
    ///     fn heal(){
    ///         health.clamped_add(100, 1)
    ///     }
    pub fn clamped_add(&self, max: u8, rhs: u8) {
        self.set(self.get().saturating_add(rhs).clamp(0, max));
    }
}

// NOTE: operators (+, -, *, /) not very useful due to the need to borrow cel (not Copy).

// ---------- Option

pub trait CelOptionExt<T> {
    fn unwrap_or_default(&self) -> T;
    fn take(&self) -> Option<T>;
}

// ---------- fmt

impl<T> Debug for Cel<T>
where
    T: Copy + Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.get().fmt(f)
    }
}

impl<T> Display for Cel<T>
where
    T: Copy + Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.get().fmt(f)
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use googletest::prelude::*;

    #[gtest]
    fn eq() {
        let a = 42.cel();

        expect_eq!(a, 42);
        expect_true!(a == 42);

        expect_eq!(a, a);
        expect_true!(a == a);

        expect_eq!(a, &42);
        expect_true!(a == &42);
    }

    #[gtest]
    fn set_owned() {
        let a = 0.cel();
        expect_eq!(a, 0);

        a.set(43);
        expect_eq!(a, 43);
    }

    #[gtest]
    fn set_ref() {
        let a = 0.cel();
        expect_eq!(a, 0);

        a.set(&43);
        expect_eq!(a, 43);
    }

    #[gtest]
    fn set_cel() {
        let a = 0.cel();
        expect_eq!(a, 0);

        let b = 42.cel();
        expect_eq!(b, 42);

        a.set(&b);
        expect_eq!(a, 42);
    }

    #[gtest]
    fn add() {
        let a = 1.cel();
        a.inc(2);
        expect_eq!(a, 3);

        a.inc(&4);
        expect_eq!(a, 7);

        a.inc(&5.cel());
        expect_eq!(a, 12);
    }

    #[gtest]
    fn inc() {
        let a = 1.cel();
        a.inc(1);
        expect_eq!(a, 2);
    }

    #[gtest]
    fn saturating_sub() {
        let a = 2u8.cel();
        expect_eq!(a, 2);
        a.saturating_sub(1);
        expect_eq!(a, 1);
        a.saturating_sub(1);
        expect_eq!(a, 0);
        a.saturating_sub(1);
        expect_eq!(a, 0);
    }
}
