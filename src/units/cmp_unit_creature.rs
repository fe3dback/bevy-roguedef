use bevy::prelude::{Component, Reflect};
use brg_core::prelude::types::Speed;
use brg_core::prelude::V2;

use super::cmp_unit::CmpUnit;

// CmpUnitCreature is base class for all movable units
#[derive(Component, Debug, Reflect, Default)]
#[require(CmpUnit, CmpUnitMovement)]
pub struct CmpUnitCreature {}

#[derive(Component, Debug, Reflect, Default)]
#[require(CmpUnitMovementInput)]
pub struct CmpUnitMovement {
    pub speed: Speed,
}

#[derive(Component, Debug, Reflect, Default)]
pub struct CmpUnitMovementInput {
    pub direction_vector: V2,
}
