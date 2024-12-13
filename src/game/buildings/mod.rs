pub mod plug;
pub mod sup;
mod sys;

use bevy::prelude::{Component, Reflect};

use crate::components::unit::CmpUnit;
use crate::components::unit_creature::CmpUnitBuilding;

#[derive(Component, Reflect, Default)]
#[require(CmpUnitBuilding)]
pub struct CmpBuildingElectricity {}
