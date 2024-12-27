use bevy::app::App;
use bevy::prelude::Plugin;
use bevy_trait_query::RegisterExt;

use crate::components::movement::CmpMovement;
use crate::components::unit::CmpUnit;
use crate::components::unit_creature::{CmpUnitBuilding, CmpUnitCreature};
use crate::components::unit_creature_player::CmpUnitCreaturePlayer;
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
