use crate::prelude::*;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct RGB(pub [u8; 3]);

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct RGBA(pub [u8; 4]);

impl RGB {
    pub const BLACK: Self = Self([0, 0, 0]);
    pub const RED: Self = Self([255, 0, 0]);
    pub const YELLOW: Self = Self([255, 255, 0]);
    pub const GREEN: Self = Self([0, 255, 0]);
    pub const BLUE: Self = Self([0, 0, 255]);
    pub const WHITE: Self = Self([255, 255, 255]);

    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self([r, g, b])
    }

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
        let RGB([r, g, b]) = *self;
        RGBA([r, g, b, a])
    }
}

impl RGBA {
    pub const BLACK: Self = Self([0, 0, 0, 255]);
    pub const RED: Self = Self([255, 0, 0, 255]);
    pub const YELLOW: Self = Self([255, 255, 0, 255]);
    pub const GREEN: Self = Self([0, 255, 0, 255]);
    pub const BLUE: Self = Self([0, 0, 255, 255]);
    pub const WHITE: Self = Self([255, 255, 255, 255]);
    pub const TRANSPARENT: Self = Self([0, 0, 0, 0]);

    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self([r, g, b, a])
    }

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
        let RGBA([r, g, b, _]) = *self;
        RGBA([r, g, b, a])
    }
}
