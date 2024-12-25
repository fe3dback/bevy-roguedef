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

impl EArchetype {
    pub fn graph_priority(self) -> u32 {
        match self {
            EArchetype::Source => 100,
            EArchetype::Castle => 50,
            EArchetype::Pole => 10,
            EArchetype::Tower => 1,
        }
    }
}
