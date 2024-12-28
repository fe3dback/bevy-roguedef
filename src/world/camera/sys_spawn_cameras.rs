use bevy::core::Name;
use bevy::math::Vec3;
use bevy::prelude::{Camera3d, Commands, StateScoped, Transform};
use bevy_flycam::FlyCam;
use brg_scene::prelude::InGame;

use crate::world::camera::cmp_active_camera::CmpMarkerActiveCamera;

pub fn spawn_cameras(mut cmd: Commands) {
    cmd.spawn((
        StateScoped(InGame),
        Name::from("Editor Camera"),
        CmpMarkerActiveCamera,
        Camera3d::default(),
        Transform::from_xyz(4.0, 4.0, 8.0).looking_at(Vec3::ZERO, Vec3::Y),
        FlyCam,
    ));
}
