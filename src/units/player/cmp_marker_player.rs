use bevy::prelude::Component;
use bevy::reflect::Reflect;

use crate::units::cmp_unit_creature::CmpUnitCreature;

#[derive(Component, Debug, Reflect)]
#[require(CmpUnitCreature)]
pub struct CmpMarkerPlayer;
