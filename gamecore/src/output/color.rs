use crate::prelude::*;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct RGB(vec3u8);

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct RGBA(vec4u8);

impl RGB {
    pub const BLACK: Self = Self(vec3(0, 0, 0));

    pub fn r(&self) -> u8 {
        self.0[0]
    }
    pub fn g(&self) -> u8 {
        self.0[1]
    }
    pub fn b(&self) -> u8 {
        self.0[2]
    }
    pub fn hex(&self) -> String{
        format!("#{:02x}{:02x}{:02x}", self.r(), self.g(), self.b())
    }
}

impl RGBA {
    pub const BLACK: Self = Self(vec4(0, 0, 0, 255));
    pub const WHITE: Self = Self(vec4(255,255, 255, 255));
    pub const TRANSPARENT: Self = Self(vec4(0, 0, 0, 0));

    pub fn r(&self) -> u8 {
        self.0[0]
    }
    pub fn g(&self) -> u8 {
        self.0[1]
    }
    pub fn b(&self) -> u8 {
        self.0[2]
    }
    pub fn a(&self) -> u8 {
        self.0[3]
    }
    pub fn hex(&self) -> String{
        format!("#{:02x}{:02x}{:02x}{:02x}", self.r(), self.g(), self.b(), self.a())
    }
}
