use bevy::app::App;
use bevy::prelude::Plugin;

use crate::components::movement::CmpMovement;
use crate::components::transform::CmpTransform2D;
use crate::components::unit::CmpUnit;
use crate::components::unit_creature::{CmpUnitBuilding, CmpUnitCreature};
use crate::components::unit_creature_player::CmpUnitCreaturePlayer;

pub struct Plug {}

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app
            //
            .register_type::<CmpTransform2D>()
            .register_type::<CmpMovement>()
            .register_type::<CmpUnit>()
            .register_type::<CmpUnitBuilding>()
            .register_type::<CmpUnitCreature>()
            .register_type::<CmpUnitCreaturePlayer>();
    }
}
