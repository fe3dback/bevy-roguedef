use bevy::app::{App, Plugin};
use bevy::prelude::{IntoSystemConfigs, OnEnter, Update};
use brg_scene::prelude::{has_feature, GameSystemSet, Loaded, SceneFeature};

use crate::units::player::sys_movement_input::update_movement_input;
use crate::units::player::sys_spawn_player::spawn_player;
use crate::units::player::sys_weapon_fire::weapon_trigger_fire;

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app
            //
            .add_systems(OnEnter(Loaded), spawn_player.in_set(GameSystemSet::NOT_ON_PAUSE__SpawnPlayerStaff).run_if(has_feature(SceneFeature::Units)))
            .add_systems(Update, update_movement_input.in_set(GameSystemSet::NOT_ON_PAUSE__ProcessInput))
            .add_systems(Update, weapon_trigger_fire.in_set(GameSystemSet::NOT_ON_PAUSE__ProcessInput))
        //-
        ;
    }
}
