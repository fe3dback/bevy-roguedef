use bevy::app::{App, Plugin};
use bevy::prelude::{IntoSystemConfigs, OnEnter, Update};
use brg_scene::prelude::GameState::Loading;
use brg_scene::prelude::{GameSystemSet, InGame};

use crate::world::camera::cmp::CmpCameraAutoFollowSettings;
use crate::world::camera::sys_spawn_cameras::{spawn_cameras, spawn_default_loading_camera};
use crate::world::camera::sys_update_game_camera::update_game_camera_position;

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app
            //
            .register_type::<CmpCameraAutoFollowSettings>()
            //
            // .add_plugins(NoCameraPlayerPlugin) // todo
            // .insert_resource(MovementSettings {
            //     sensitivity: 0.00012,
            //     speed: 12.0,
            // })
            // .insert_resource(KeyBindings {
            //     move_ascend: KeyCode::Space,
            //     move_descend: KeyCode::ShiftLeft,
            //     ..Default::default()
            // })
            .add_systems(OnEnter(Loading), spawn_default_loading_camera.in_set(GameSystemSet::LoadingSystem))
            .add_systems(OnEnter(InGame), spawn_cameras.in_set(GameSystemSet::InGameSpawnWorldEnvironment))
            .add_systems(Update, update_game_camera_position.in_set(GameSystemSet::InGameUpdateCameras))
        //-
        ;
    }
}
