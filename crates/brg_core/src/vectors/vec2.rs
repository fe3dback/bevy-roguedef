use std::f32;
use std::f32::consts::PI;
use std::fmt::Display;
use std::hash::{Hash, Hasher};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use bevy::math::Rot2;
use bevy::prelude::{Dir2, Reflect, Vec2, Vec3};

use super::vec3::V3;
use crate::prelude::types::{Angle, Meter};

// Internal game vector2D struct with inverted Y axis
// and some helper functions to transform game-space vectors
// into engine-space.
#[derive(PartialEq, PartialOrd, Debug, Copy, Clone, Reflect)]
pub struct V2 {
    pub x: Meter,
    pub y: Meter,
}

impl Hash for V2 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        format!("{:.1$}", self.x, 5).hash(state);
        format!("{:.1$}", self.y, 5).hash(state);
    }
}

impl Display for V2 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "V2[{:.2},{:.2}]", self.x, self.y)
    }
}

#[allow(dead_code)]
impl V2 {
    pub const ZERO: Self = Self::splat(0.0);
    pub const ONE: Self = Self::splat(1.0);
    pub const NEG_ONE: Self = Self::splat(-1.0);
    pub const UP: Self = Self { x: 0.0, y: -1.0 };
    pub const DOWN: Self = Self { x: 0.0, y: 1.0 };
    pub const LEFT: Self = Self { x: -1.0, y: 0.0 };
    pub const RIGHT: Self = Self { x: 1.0, y: 0.0 };

    // --- basic

    #[inline(always)]
    pub const fn new(x: Meter, y: Meter) -> Self {
        Self { x, y }
    }

    #[inline(always)]
    pub const fn splat(f: Meter) -> Self {
        Self { x: f, y: f }
    }

    #[inline]
    pub fn with_height(&self, height: Meter) -> V3 {
        V3 {
            x: self.x,
            y: self.y,
            h: height,
        }
    }

    // --- from/to engine vectors

    #[inline(always)]
    pub const fn new_2d(x: Meter, y: Meter) -> Vec2 {
        Vec2 { x, y }
    }

    #[inline(always)]
    pub const fn new_3d(x: Meter, y: Meter) -> Vec3 {
        Vec3 { x, y, z: 0.0 }
    }

    #[inline(always)]
    pub fn from_2d(vec: Vec2) -> Self {
        Self {
            x: vec.x,
            y: -vec.y,
        }
    }

    #[inline(always)]
    pub fn from_2d_ui(vec: Vec2) -> Self {
        Self { x: vec.x, y: vec.y }
    }

    #[inline(always)]
    pub fn from_3d(vec: Vec3) -> Self {
        let vec = V3::from_3d(vec);

        Self { x: vec.x, y: vec.y }
    }

    #[inline(always)]
    pub fn as_2d(&self) -> Vec2 {
        Vec2 {
            x: self.x,
            y: -self.y,
        }
    }

    #[inline(always)]
    pub fn as_2d_ui(&self) -> Vec2 {
        Vec2 {
            x: self.x,
            y: self.y,
        }
    }

    #[inline(always)]
    pub fn as_3d(&self) -> Vec3 {
        V3::new(self.x, self.y, 0.0).as_3d()
    }

    #[inline(always)]
    pub fn as_3d_ui(&self) -> Vec3 {
        V3::new(self.x, -self.y, 0.0).as_3d()
    }

    // --- simple math

    /// Returns a vector containing the nearest integer to a number for each element of `self`.
    /// Round half-way cases away from 0.0.
    #[inline]
    pub fn round(self) -> Self {
        Self {
            x: f32::round(self.x),
            y: f32::round(self.y),
        }
    }

    /// Returns a vector containing the largest integer less than or equal to a number for each
    /// element of `self`.
    #[inline]
    pub fn floor(self) -> Self {
        Self {
            x: f32::floor(self.x),
            y: f32::floor(self.y),
        }
    }

    /// Returns a vector containing the smallest integer greater than or equal to a number for
    /// each element of `self`.
    #[inline]
    pub fn ceil(self) -> Self {
        Self {
            x: f32::ceil(self.x),
            y: f32::ceil(self.y),
        }
    }

    #[inline]
    pub fn min(a: Self, b: Self) -> Self {
        Self {
            x: f32::min(a.x, b.x),
            y: f32::min(a.y, b.y),
        }
    }

    #[inline]
    pub fn max(a: Self, b: Self) -> Self {
        Self {
            x: f32::max(a.x, b.x),
            y: f32::max(a.y, b.y),
        }
    }

    #[inline]
    pub fn lerp(a: Self, b: Self, time: f32) -> Self {
        a + ((b - a) * time)
    }

    /// Component-wise clamping of values, similar to [`f32::clamp`].
    ///
    /// Each element in `min` must be less-or-equal to the corresponding element in `max`.
    #[inline]
    pub fn clamp(self, a: Self, b: Self) -> Self {
        match a <= b {
            true => Self::min(Self::max(self, a), b),
            false => Self::min(Self::max(self, b), a),
        }
    }

    /// Returns a vector with a length no less than `min` and no more than `max`
    pub fn clamp_length(self, min: f32, max: f32) -> Self {
        let (a, b) = (min, max);
        let min = a.min(b);
        let max = a.max(b);

        let length_sq = self.dot(self);
        if length_sq < min * min {
            min * (self / f32::sqrt(length_sq))
        } else if length_sq > max * max {
            max * (self / f32::sqrt(length_sq))
        } else {
            self
        }
    }

    /// Returns a vector with a length no more than `max`
    pub fn clamp_length_max(self, max: f32) -> Self {
        let length_sq = self.dot(self);
        if length_sq > max * max {
            max * (self / f32::sqrt(length_sq))
        } else {
            self
        }
    }

    /// Returns a vector with a length no less than `min`
    pub fn clamp_length_min(self, min: f32) -> Self {
        let length_sq = self.dot(self);
        if length_sq < min * min {
            min * (self / f32::sqrt(length_sq))
        } else {
            self
        }
    }

    #[inline]
    pub fn distance(self, to: Self) -> Meter {
        f32::sqrt((self.x - to.x) * (self.x - to.x) + (self.y - to.y) * (self.y - to.y))
    }

    #[inline]
    pub fn direction(self) -> Angle {
        f32::atan2(-self.y, self.x)
    }

    #[inline]
    pub fn angle_to(self, to: Self) -> Angle {
        f32::atan2(to.y - self.y, self.x - to.x) + PI
    }

    #[inline]
    pub fn as_norm_dir_to(self, to: Self) -> V2 {
        V2::ZERO.polar_offset(1.0, self.angle_to(to))
    }

    #[inline]
    pub fn as_dir2_to(self, to: Self) -> Dir2 {
        Dir2::new_unchecked(V2::ZERO.polar_offset(1.0, self.angle_to(to)).as_2d())
    }

    #[inline]
    pub fn as_rot2(self, to: Self) -> Rot2 {
        Rot2::radians(self.angle_to(to))
    }

    #[inline]
    pub fn angle_between(self, to: Self) -> Angle {
        f32::atan2(self.cross(to), self.dot(to))
    }

    #[inline]
    pub fn rotate(self, angle: Angle) -> V2 {
        let sin = f32::sin(angle);
        let cos = f32::cos(angle);

        V2::new(self.x * cos - self.y * sin, -(self.x * sin + self.y * cos))
    }

    // --- algebra

    /// Computes the cross between `self` and `rhs`.
    #[inline]
    pub fn cross(self, to: Self) -> f32 {
        self.x * to.y - self.y * to.x
    }

    /// Computes the dot product of `self` and `rhs`.
    #[inline]
    pub fn dot(self, rhs: Self) -> f32 {
        (self.x * rhs.x) + (self.y * rhs.y)
    }

    /// Computes the length of `self`.
    #[inline]
    pub fn length(self) -> f32 {
        f32::sqrt(self.dot(self))
    }

    /// Returns `self` normalized to length 1.0.
    /// if vector length is zero, this function return ZERO vector as result
    #[inline]
    pub fn normalize(self) -> Self {
        let length = self.length();
        if length <= f32::EPSILON {
            return Self::ZERO;
        }

        self * (1.0 / length)
    }

    // --- trigonometry

    #[inline]
    pub fn angle_between_vectors(a: Self, b: Self) -> Angle {
        (a.y - b.y).atan2(b.x - a.x)
    }

    #[inline]
    pub fn polar_offset(self, distance: Meter, angle: Angle) -> V2 {
        V2::new(
            self.x + distance * f32::cos(angle),
            self.y - distance * f32::sin(angle),
        )
    }
}

impl Default for V2 {
    #[inline(always)]
    fn default() -> Self {
        Self::ZERO
    }
}

impl Eq for V2 {
}

impl Div<V2> for V2 {
    type Output = Self;
    #[inline]
    fn div(self, rhs: Self) -> Self {
        Self {
            x: self.x.div(rhs.x),
            y: self.y.div(rhs.y),
        }
    }
}

impl Div<f32> for V2 {
    type Output = Self;
    #[inline]
    fn div(self, rhs: f32) -> Self {
        Self {
            x: self.x.div(rhs),
            y: self.y.div(rhs),
        }
    }
}

impl Div<V2> for f32 {
    type Output = V2;
    #[inline]
    fn div(self, rhs: V2) -> V2 {
        V2 {
            x: self.div(rhs.x),
            y: self.div(rhs.y),
        }
    }
}

impl DivAssign<V2> for V2 {
    #[inline]
    fn div_assign(&mut self, rhs: Self) {
        self.x.div_assign(rhs.x);
        self.y.div_assign(rhs.y);
    }
}

impl DivAssign<f32> for V2 {
    #[inline]
    fn div_assign(&mut self, rhs: f32) {
        self.x.div_assign(rhs);
        self.y.div_assign(rhs);
    }
}

impl Mul<V2> for V2 {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Self) -> Self {
        Self {
            x: self.x.mul(rhs.x),
            y: self.y.mul(rhs.y),
        }
    }
}

impl Mul<f32> for V2 {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: f32) -> Self {
        Self {
            x: self.x.mul(rhs),
            y: self.y.mul(rhs),
        }
    }
}

impl Mul<V2> for f32 {
    type Output = V2;
    #[inline]
    fn mul(self, rhs: V2) -> V2 {
        V2 {
            x: self.mul(rhs.x),
            y: self.mul(rhs.y),
        }
    }
}

impl MulAssign<V2> for V2 {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        self.x.mul_assign(rhs.x);
        self.y.mul_assign(rhs.y);
    }
}

impl MulAssign<f32> for V2 {
    #[inline]
    fn mul_assign(&mut self, rhs: f32) {
        self.x.mul_assign(rhs);
        self.y.mul_assign(rhs);
    }
}

impl Add<V2> for V2 {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x.add(rhs.x),
            y: self.y.add(rhs.y),
        }
    }
}

impl Add<f32> for V2 {
    type Output = Self;
    #[inline]
    fn add(self, rhs: f32) -> Self {
        Self {
            x: self.x.add(rhs),
            y: self.y.add(rhs),
        }
    }
}

impl Add<V2> for f32 {
    type Output = V2;
    #[inline]
    fn add(self, rhs: V2) -> V2 {
        V2 {
            x: self.add(rhs.x),
            y: self.add(rhs.y),
        }
    }
}

impl AddAssign<V2> for V2 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.x.add_assign(rhs.x);
        self.y.add_assign(rhs.y);
    }
}

impl AddAssign<f32> for V2 {
    #[inline]
    fn add_assign(&mut self, rhs: f32) {
        self.x.add_assign(rhs);
        self.y.add_assign(rhs);
    }
}

impl Sub<V2> for V2 {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x.sub(rhs.x),
            y: self.y.sub(rhs.y),
        }
    }
}

impl Sub<f32> for V2 {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: f32) -> Self {
        Self {
            x: self.x.sub(rhs),
            y: self.y.sub(rhs),
        }
    }
}

impl Sub<V2> for f32 {
    type Output = V2;
    #[inline]
    fn sub(self, rhs: V2) -> V2 {
        V2 {
            x: self.sub(rhs.x),
            y: self.sub(rhs.y),
        }
    }
}

impl SubAssign<V2> for V2 {
    #[inline]
    fn sub_assign(&mut self, rhs: V2) {
        self.x.sub_assign(rhs.x);
        self.y.sub_assign(rhs.y);
    }
}

impl SubAssign<f32> for V2 {
    #[inline]
    fn sub_assign(&mut self, rhs: f32) {
        self.x.sub_assign(rhs);
        self.y.sub_assign(rhs);
    }
}
