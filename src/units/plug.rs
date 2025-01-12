use bevy::app::{App, Plugin};
use bevy::prelude::{IntoSystemConfigs, Update};
use brg_scene::prelude::{has_feature, GameSystemSet, SceneFeature};

use super::cmp_team::CmpTeam;
use super::cmp_unit_creature::CmpUnitMovementInput;
use super::sys_movement::update_unit_movement;

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app
            //
            .add_plugins(super::player::plug::Plug)
            .add_plugins(super::mobs::plug::Plug)
            .add_plugins(super::weapon::plug::Plug)
            .add_plugins(super::stats::plug::Plug)
            .add_plugins(super::spells::plug::Plug)
            .add_plugins(super::ai::plug::Plug)
            //
            .register_type::<CmpUnitMovementInput>()
            .register_type::<CmpTeam>()
            //
            .add_systems(Update, update_unit_movement.in_set(GameSystemSet::InGame_NOPAUSE_UpdateMovements).run_if(has_feature(SceneFeature::Units)))
        //-
        ;
    }
}
