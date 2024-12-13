use bevy::prelude::{Component, Reflect};

use crate::components::movement::CmpMovement;
use crate::components::unit::CmpUnit;
use crate::game::damage::CmpHealth;

#[derive(Component, Reflect, Default)]
#[require(CmpUnit, CmpMovement, CmpHealth)]
pub struct CmpUnitCreature {}

#[derive(Component, Reflect, Default)]
#[require(CmpUnit, CmpHealth)]
pub struct CmpUnitBuilding {}
