use bevy::prelude::{Component, Reflect};
use brg_core::prelude::V2;

use crate::core::measurements::Speed;
use crate::units::cmp_unit::CmpUnit;

// CmpUnitCreature is base class for all movable units
#[derive(Component, Debug, Reflect, Default)]
#[require(CmpUnit, CmpUnitMovementInput)]
pub struct CmpUnitCreature {}

// CmpUnitCreature is base class for all movable units
#[derive(Component, Debug, Reflect, Default)]
pub struct CmpUnitMovementInput {
    pub direction_vector: V2,
    pub speed:            Speed,
}
