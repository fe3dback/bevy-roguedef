use bevy::app::{App, Plugin};
use bevy::prelude::{IntoSystemConfigs, Update};
use brg_scene::prelude::{has_feature, GameSystemSet, SceneFeature};

use crate::units::cmp_unit_creature::CmpUnitMovementInput;
use crate::units::sys_movement::update_unit_movement;

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app
            //
            .add_plugins(super::player::plug::Plug)
            //
            .register_type::<CmpUnitMovementInput>()
            //
            .add_systems(Update, update_unit_movement.in_set(GameSystemSet::InGameUpdateMovements).run_if(has_feature(SceneFeature::Units)))
        //-
        ;
    }
}
