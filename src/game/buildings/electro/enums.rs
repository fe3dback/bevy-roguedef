use bevy::prelude::Reflect;
use serde::Deserialize;

pub enum EChargeDirection {
    Out,
    In,
}

#[derive(Copy, Clone, Reflect, Debug, Default, PartialEq, Eq, Hash, Deserialize)]
pub enum EArchetype {
    #[default]
    Pole,
    Tower,
    Castle,
    Source,
}
