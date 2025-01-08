use bevy::app::{App, Plugin};
use bevy::prelude::{IntoSystemConfigs, Update};
use brg_scene::prelude::GameSystemSet;

use super::cmp::CmpTimeToLife;
use super::sys::remove_expired_ttl_entities;

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app
        //
            .register_type::<CmpTimeToLife>()
            .add_systems(Update, remove_expired_ttl_entities.in_set(GameSystemSet::InGame_NOPAUSE_DespawnObjects))
        //-
        ;
    }
}
