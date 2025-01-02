use bevy::core::Name;
use bevy::prelude::{
    default,
    Camera,
    Camera3d,
    Commands,
    PerspectiveProjection,
    Projection,
    Res,
    StateScoped,
    Transform,
    Vec3,
};
use brg_core::prelude::{ANGLE60, V2};
use brg_fundamental::prelude::CmpTransform2D;
use brg_scene::prelude::GameState::Loading;
use brg_scene::prelude::InGame;

use crate::world::camera::cmp::{CmpCameraAutoFollowSettings, CmpMarkerCameraActive};
use crate::world::camera::enums::CmpCameraType;
use crate::world::camera::res::ResCameraSettings;

pub fn spawn_default_loading_camera(mut cmd: Commands) {
    cmd.spawn((
        StateScoped(Loading),
        Name::from("Loading Camera"),
        CmpMarkerCameraActive,
        Camera3d::default(),
    ));
}

pub fn spawn_cameras(mut cmd: Commands, settings: Res<ResCameraSettings>) {
    let cam_editor_fly = cmd
        .spawn((
            StateScoped(InGame),
            Name::from("Editor Fly Camera"),
            CmpCameraType::EditorFly,
            Camera3d::default(),
            Camera {
                is_active: settings.active == CmpCameraType::EditorFly,
                ..default()
            },
            Transform::from_xyz(4.0, 4.0, 8.0).looking_at(Vec3::ZERO, Vec3::Y),
        ))
        .id();

    let cam_game_top_down = cmd
        .spawn((
            StateScoped(InGame),
            Name::from("Game TopDown Camera"),
            CmpCameraType::GameTopDown,
            CmpCameraAutoFollowSettings {
                offset:     V2::new(0.0, 14.0),
                snap_speed: 4.0,
            },
            Camera3d::default(),
            Camera {
                is_active: settings.active == CmpCameraType::GameTopDown,
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
        ))
        .id();

    if settings.active == CmpCameraType::EditorFly {
        cmd.entity(cam_editor_fly).insert(CmpMarkerCameraActive);
    }

    if settings.active == CmpCameraType::GameTopDown {
        cmd.entity(cam_game_top_down).insert(CmpMarkerCameraActive);
    }
}
