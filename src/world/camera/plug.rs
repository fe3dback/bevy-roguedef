use bevy::app::{App, Plugin};
use bevy::prelude::{IntoSystemConfigs, OnEnter, Update};
use brg_scene::prelude::GameState::Loading;
use brg_scene::prelude::{GameSystemSet, InGame};

use crate::world::camera::cmp::CmpCameraAutoFollowSettings;
use crate::world::camera::res::ResCameraSettings;
use crate::world::camera::sys_editor_fly::{
    editor_fly_lock_cursor,
    editor_fly_look_and_move,
    editor_fly_toggle_mouse_lock,
};
use crate::world::camera::sys_spawn_cameras::{spawn_cameras, spawn_default_loading_camera};
use crate::world::camera::sys_switch_camera::{
    switch_camera_on_keyboard_input,
    switch_camera_on_settings_change,
};
use crate::world::camera::sys_update_game_camera::update_game_camera_position;

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app
            //
            .register_type::<CmpCameraAutoFollowSettings>()
            .register_type::<ResCameraSettings>()
            //
            .insert_resource(ResCameraSettings::default())
            //
            .add_systems(OnEnter(Loading), spawn_default_loading_camera.in_set(GameSystemSet::LoadingSystem))
            .add_systems(OnEnter(InGame), spawn_cameras.in_set(GameSystemSet::InGameSpawnWorldEnvironment))
            .add_systems(Update, (
                switch_camera_on_keyboard_input,
                switch_camera_on_settings_change,
                editor_fly_toggle_mouse_lock,
                editor_fly_lock_cursor,
                editor_fly_look_and_move,
                update_game_camera_position,
            ).in_set(GameSystemSet::InGameUpdateCameras))
        //-
        ;
    }
}
