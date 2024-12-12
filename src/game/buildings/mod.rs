pub mod plug;
pub mod sup;
mod sys;

use {
    crate::components::unit::CmpUnit,
    bevy::prelude::{Component, Reflect},
};

#[derive(Component, Reflect, Default)]
#[require(CmpUnit)]
pub struct CmpBuilding {}
