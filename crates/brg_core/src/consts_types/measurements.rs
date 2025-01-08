use std::ops::{Add, AddAssign, Mul, MulAssign};

use bevy::prelude::Reflect;

use crate::prelude::types::{KiloMeter, Meter};

#[derive(Debug, Reflect, Copy, Clone)]
pub enum Speed {
    // kilometers per hour
    KMH(KiloMeter),
    // meters per second
    MPS(Meter),
}

impl Default for Speed {
    fn default() -> Self {
        Self::KMH(1.0)
    }
}

impl Speed {
    pub fn meters_per_second(&self) -> Meter {
        match self {
            Self::KMH(kmh) => kmh * 1000.0 / 3600.0,
            Self::MPS(mps) => *mps,
        }
    }

    pub fn km_per_hour(&self) -> KiloMeter {
        match self {
            Self::KMH(kmh) => *kmh,
            Self::MPS(mps) => mps * 3600.0 / 1000.0,
        }
    }
}

#[auto_impl_ops::auto_ops]
impl AddAssign for Speed {
    fn add_assign(&mut self, other: Self) {
        match self {
            Self::KMH(kmh) => kmh.add_assign(other.km_per_hour()),
            Self::MPS(mps) => mps.add_assign(other.meters_per_second()),
        }
    }
}

#[auto_impl_ops::auto_ops]
impl MulAssign for Speed {
    fn mul_assign(&mut self, other: Self) {
        match self {
            Self::KMH(kmh) => kmh.mul_assign(other.km_per_hour()),
            Self::MPS(mps) => mps.mul_assign(other.meters_per_second()),
        }
    }
}

#[auto_impl_ops::auto_ops]
impl MulAssign<f32> for Speed {
    fn mul_assign(&mut self, other: f32) {
        match self {
            Self::KMH(kmh) => kmh.mul_assign(other),
            Self::MPS(mps) => mps.mul_assign(other),
        }
    }
}
