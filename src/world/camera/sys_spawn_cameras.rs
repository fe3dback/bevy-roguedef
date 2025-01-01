use bevy::core::Name;
use bevy::prelude::{
    default,
    Camera,
    Camera3d,
    Commands,
    PerspectiveProjection,
    Projection,
    StateScoped,
};
use brg_core::prelude::{ANGLE60, V2};
use brg_fundamental::prelude::CmpTransform2D;
use brg_scene::prelude::GameState::Loading;
use brg_scene::prelude::InGame;

use crate::world::camera::cmp::{
    CmpCameraAutoFollowSettings,
    CmpMarkerCameraActive,
    CmpMarkerCameraTypeGameTopDown,
};

pub fn spawn_default_loading_camera(mut cmd: Commands) {
    cmd.spawn((
        StateScoped(Loading),
        Name::from("Loading Camera"),
        CmpMarkerCameraActive,
        Camera3d::default(),
    ));
}

pub fn spawn_cameras(mut cmd: Commands) {
    // todo;
    // cmd.spawn((
    //     StateScoped(InGame),
    //     Name::from("Editor Fly Camera"),
    //     CmpMarkerCameraTypeEditorFly,
    //     FlyCam,
    //     Camera3d::default(),
    //     Camera {
    //         is_active: false,
    //         ..default()
    //     },
    //     Transform::from_xyz(4.0, 4.0, 8.0).looking_at(Vec3::ZERO, Vec3::Y),
    // ));

    cmd.spawn((
        StateScoped(InGame),
        Name::from("Game TopDown Camera"),
        CmpMarkerCameraActive,
        CmpMarkerCameraTypeGameTopDown,
        CmpCameraAutoFollowSettings {
            offset:     V2::new(0.0, 14.0),
            snap_speed: 4.0,
        },
        Camera3d::default(),
        Camera {
            is_active: true,
            ..default()
        },
        Projection::Perspective(PerspectiveProjection {
            fov: 45.0_f32.to_radians(),
            ..default()
        }),
        CmpTransform2D {
            height: 25.0,
            yaw: ANGLE60,
            ..default()
        },
    ));
}
