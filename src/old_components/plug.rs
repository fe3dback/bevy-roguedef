use bevy::app::App;
use bevy::prelude::Plugin;
use bevy_trait_query::RegisterExt;

use crate::components_old::movement::CmpMovement;
use crate::components_old::unit::CmpUnit;
use crate::components_old::unit_creature::{CmpUnitBuilding, CmpUnitCreature};
use crate::components_old::unit_creature_player::CmpUnitCreaturePlayer;
use crate::game::energy::CmpEnergyContainer;

pub struct Plug {}

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app
            //
            .register_type::<CmpMovement>()
            .register_type::<CmpUnit>()
            .register_type::<CmpUnitBuilding>()
            .register_type::<CmpUnitCreature>()
            .register_type::<CmpUnitCreaturePlayer>()
            //
            .register_component_as::<dyn CmpEnergyContainer, CmpUnitCreature>()
        //~
        ;
    }
}
