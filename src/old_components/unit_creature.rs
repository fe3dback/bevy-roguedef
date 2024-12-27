use bevy::prelude::{Component, Reflect};

use crate::components_old::movement::CmpMovement;
use crate::components_old::unit::CmpUnit;
use crate::game::damage::CmpHealth;
use crate::game::energy::CmpEnergyContainer;

#[derive(Component, Reflect, Default)]
#[require(CmpUnit, CmpMovement, CmpHealth)]
pub struct CmpUnitCreature {}

#[derive(Component, Reflect, Default)]
#[require(CmpUnit, CmpHealth)]
pub struct CmpUnitBuilding {}

impl CmpEnergyContainer for CmpUnitCreature {
    fn try_spend(&mut self, _: f32) -> bool {
        // creatures has limitless battery
        true
    }
}
