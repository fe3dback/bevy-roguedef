use std::ops::{AddAssign, *};

use bevy::prelude::{Reflect, Vec3};

// Internal game vector3D struct with inverted Y axis
// and some helper functions to transform game-space vectors
// into engine-space.
#[derive(PartialEq, PartialOrd, Debug, Copy, Clone, Reflect)]
pub struct V3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[allow(dead_code)]
impl V3 {
    pub const ZERO: V3 = V3::splat(0.0);
    pub const ONE: V3 = V3::splat(1.0);

    // --- basic

    #[inline(always)]
    pub const fn new(x: f32, y: f32, z: f32) -> V3 {
        V3 { x, y, z }
    }

    #[inline(always)]
    pub const fn splat(f: f32) -> V3 {
        V3 { x: f, y: f, z: f }
    }

    // --- from/to engine vectors

    #[inline(always)]
    pub const fn new_3d(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
    }

    #[inline(always)]
    pub fn from_3d(vec: Vec3) -> Self {
        Self {
            x: vec.x,
            y: -vec.y,
            z: vec.z,
        }
    }

    #[inline(always)]
    pub fn as_3d(&self) -> Vec3 {
        Vec3 {
            x: self.x,
            y: -self.y,
            z: self.z,
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
        self.z = &self.z + &other.z;
    }
}

#[auto_impl_ops::auto_ops]
impl SubAssign<&V3> for V3 {
    fn sub_assign(&mut self, other: &Self) {
        self.x = &self.x - &other.x;
        self.y = &self.y - &other.y;
        self.z = &self.z - &other.z;
    }
}
