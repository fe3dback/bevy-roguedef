use crate::components::movement::CmpMovement;
use crate::components::unit::CmpUnit;
use bevy::prelude::{Component, Reflect};

#[derive(Component, Reflect, Default)]
#[require(CmpUnit, CmpMovement)]
pub struct CmpUnitCreature {}