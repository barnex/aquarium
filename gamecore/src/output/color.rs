use crate::prelude::*;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct RGB(pub vec3u8);

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct RGBA(pub vec4u8);

impl RGB {
    pub const BLACK: Self = Self(vec3(0, 0, 0));
    pub const RED: Self = Self(vec3(255, 0, 0));
    pub const YELLOW: Self = Self(vec3(255, 255, 0));
    pub const GREEN: Self = Self(vec3(0, 255, 0));
    pub const BLUE: Self = Self(vec3(0, 0, 255));
    pub const WHITE: Self = Self(vec3(255, 255, 255));

    pub fn r(&self) -> u8 {
        self.0[0]
    }
    pub fn g(&self) -> u8 {
        self.0[1]
    }
    pub fn b(&self) -> u8 {
        self.0[2]
    }
    pub fn hex(&self) -> String {
        format!("#{:02x}{:02x}{:02x}", self.r(), self.g(), self.b())
    }
    pub fn with_alpha(&self, a: u8) -> RGBA {
        let RGB(Vector([r, g, b])) = *self;
        RGBA(Vector([r, g, b, a]))
    }
}

impl RGBA {
    pub const BLACK: Self = Self(vec4(0, 0, 0, 255));
    pub const RED: Self = Self(vec4(255, 0, 0, 255));
    pub const YELLOW: Self = Self(vec4(255, 255, 0, 255));
    pub const GREEN: Self = Self(vec4(0, 255, 0, 255));
    pub const BLUE: Self = Self(vec4(0, 0, 255, 255));
    pub const WHITE: Self = Self(vec4(255, 255, 255, 255));
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
    pub fn hex(&self) -> String {
        format!("#{:02x}{:02x}{:02x}{:02x}", self.r(), self.g(), self.b(), self.a())
    }
    pub fn with_alpha(&self, a: u8) -> RGBA {
        let RGBA(Vector([r, g, b, _])) = *self;
        RGBA(Vector([r, g, b, a]))
    }
}
