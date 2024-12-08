use {
    crate::components::{
        movement::CmpMovement,
        transform::CmpTransform2D,
        unit::CmpUnit,
        unit_creature::CmpUnitCreature,
        unit_creature_player::CmpUnitCreaturePlayer,
    },
    bevy::{app::App, prelude::Plugin},
};

pub struct Plug {}

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app
            //
            .register_type::<CmpTransform2D>()
            .register_type::<CmpMovement>()
            .register_type::<CmpUnit>()
            .register_type::<CmpUnitCreature>()
            .register_type::<CmpUnitCreaturePlayer>();
    }
}
