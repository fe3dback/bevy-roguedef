use bevy::app::{App, Plugin};
use bevy::prelude::{IntoSystemConfigs, KeyCode, OnEnter};
use bevy_flycam::{KeyBindings, MovementSettings, NoCameraPlayerPlugin};
use brg_scene::prelude::{GameSystemSet, InGame};

use crate::world::camera::sys_spawn_cameras::spawn_cameras;

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app
            //
            .add_plugins(NoCameraPlayerPlugin)
            .insert_resource(MovementSettings {
                sensitivity: 0.00012,
                speed: 12.0,
            })
            .insert_resource(KeyBindings {
                move_ascend: KeyCode::Space,
                move_descend: KeyCode::ShiftLeft,
                ..Default::default()
            })
            .add_systems(OnEnter(InGame), spawn_cameras.in_set(GameSystemSet::InGameSpawnWorldEnvironment))
        //-
        ;
    }
}
