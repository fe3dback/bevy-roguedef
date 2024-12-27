use bevy::prelude::{Component, Reflect};

use crate::components_old::unit_creature::CmpUnitCreature;

#[derive(Component, Reflect, Default)]
#[require(CmpUnitCreature)]
pub struct CmpUnitCreaturePlayer {}
