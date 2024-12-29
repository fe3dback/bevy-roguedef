use std::ops::{AddAssign, *};

use bevy::prelude::{Reflect, Vec3};

use crate::prelude::types::Meter;

// Internal game vector3D struct with inverted Y axis
// and some helper functions to transform game-space vectors
// into engine-space.
#[derive(PartialEq, PartialOrd, Debug, Copy, Clone, Reflect)]
pub struct V3 {
    pub x: Meter,
    pub y: Meter,
    pub h: Meter,
}

#[allow(dead_code)]
impl V3 {
    pub const ZERO: V3 = V3::splat(0.0);
    pub const ONE: V3 = V3::splat(1.0);

    // --- basic

    #[inline(always)]
    pub const fn new(x: Meter, y: Meter, h: Meter) -> V3 {
        V3 { x, y, h }
    }

    #[inline(always)]
    pub const fn splat(f: Meter) -> V3 {
        V3 { x: f, y: f, h: f }
    }

    // --- from/to engine vectors

    #[inline(always)]
    pub const fn new_3d(x: Meter, y: Meter, h: Meter) -> Vec3 {
        Vec3 { x, y: h, z: y }
    }

    #[inline(always)]
    pub fn from_3d(vec: Vec3) -> Self {
        Self {
            x: vec.x,
            y: vec.z,
            h: vec.y,
        }
    }

    #[inline(always)]
    pub fn as_3d(&self) -> Vec3 {
        Vec3 {
            x: self.x,
            z: self.y,
            y: self.h,
        }
    }
}

impl Default for V3 {
    #[inline(always)]
    fn default() -> Self {
        Self::ZERO
    }
}

#[auto_impl_ops::auto_ops]
impl AddAssign<&V3> for V3 {
    fn add_assign(&mut self, other: &Self) {
        self.x = &self.x + &other.x;
        self.y = &self.y + &other.y;
        self.h = &self.h + &other.h;
    }
}

#[auto_impl_ops::auto_ops]
impl SubAssign<&V3> for V3 {
    fn sub_assign(&mut self, other: &Self) {
        self.x = &self.x - &other.x;
        self.y = &self.y - &other.y;
        self.h = &self.h - &other.h;
    }
}
