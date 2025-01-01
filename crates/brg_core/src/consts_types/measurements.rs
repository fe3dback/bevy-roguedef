use bevy::prelude::Reflect;

use crate::prelude::types::{KiloMeter, Meter};

#[derive(Debug, Reflect)]
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
}
