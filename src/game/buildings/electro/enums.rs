use {bevy::prelude::Reflect, serde::Deserialize};

pub enum EChargeDirection {
    Out,
    In,
}

#[derive(Copy, Clone, Reflect, Debug, Default, PartialEq, Eq, Hash, Deserialize)]
pub enum EArchetype {
    #[default]
    Source,
    Castle,
    Pole,
    Tower,
}
