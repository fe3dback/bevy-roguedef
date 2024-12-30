use bevy::prelude::{Component, Reflect};

use crate::units::cmp_unit::CmpUnit;

// CmpUnitCreature is base class for all not-movable units (like buildings)
#[derive(Component, Debug, Reflect, Default)]
#[require(CmpUnit)]
pub struct CmpUnitBuilding {}
