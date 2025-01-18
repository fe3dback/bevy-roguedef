use std::f32::consts::PI;

use bevy::core::Name;
use bevy::prelude::{
    default,
    Camera,
    Camera3d,
    Commands,
    OrthographicProjection,
    PerspectiveProjection,
    Projection,
    Res,
    StateScoped,
    Transform,
    Vec3,
};
use bevy::render::camera::ScalingMode;
use brg_core::prelude::consts::TERRAIN_HEIGHT;
use brg_core::prelude::{ANGLE60, V2};
use brg_fundamental::prelude::{
    CmpTransform2D,
    ResCoords,
    TransformHeightKind,
    TransformMasterSlave,
};
use brg_scene::prelude::GameState::Loading;
use brg_scene::prelude::InGame;

use super::cmp::{CmpCameraAutoFollowSettings, CmpMarkerCameraActive};
use super::enums::CmpCameraType;
use super::res::ResCameraSettings;
use crate::world::landscape::cmp_actor_initiator::CmpLandscapeLoadActorInitiator;

pub fn spawn_default_loading_camera(mut cmd: Commands) {
    cmd.spawn((
        StateScoped(Loading),
        Name::from("Loading Camera"),
        CmpMarkerCameraActive,
        Camera3d::default(),
    ));
}

pub fn spawn_cameras(mut cmd: Commands, settings: Res<ResCameraSettings>, coords: Res<ResCoords>) {
    let cam_editor_fly = cmd
        .spawn((
            StateScoped(InGame),
            Name::from("Camera - Editor Fly"),
            CmpCameraType::EditorFly,
            Camera3d::default(),
            Camera {
                is_active: settings.active == CmpCameraType::EditorFly,
                ..default()
            },
            CmpLandscapeLoadActorInitiator,
            CmpTransform2D {
                // actual pos/rot will be in bevy Transform component below
                master: TransformMasterSlave::BevyTransformIsMaster,
                ..default()
            },
            Transform::from_translation(
                (coords.world_center + V2::new(0.0, 15.0))
                    .with_height(TERRAIN_HEIGHT)
                    .as_3d(),
            )
            .looking_at(coords.world_center.as_3d(), Vec3::Y),
        ))
        .id();

    let cam_editor_top_down_orthographic = cmd
        .spawn((
            StateScoped(InGame),
            Name::from("Camera - Editor TopDown Orthographic"),
            CmpCameraType::EditorTopDownOrthographic,
            Camera3d::default(),
            Camera {
                is_active: settings.active == CmpCameraType::EditorTopDownOrthographic,
                ..default()
            },
            CmpLandscapeLoadActorInitiator,
            Projection::Orthographic(OrthographicProjection {
                near: 0.1,
                far: 100.0,
                scale: 1.0,
                scaling_mode: ScalingMode::FixedHorizontal {
                    viewport_width: 100.0,
                },
                ..OrthographicProjection::default_3d()
            }),
            CmpTransform2D {
                height: 10.0,
                position: coords.world_center,
                height_kind: TransformHeightKind::AboveTerrain,
                yaw: PI / 2.0,
                ..default()
            },
        ))
        .id();

    let cam_game_top_down = cmd
        .spawn((
            StateScoped(InGame),
            Name::from("Camera - Game Strategy"),
            CmpCameraType::GameStrategy,
            CmpCameraAutoFollowSettings {
                offset:     V2::new(0.0, 14.0),
                snap_speed: 4.0,
            },
            Camera3d::default(),
            Camera {
                is_active: settings.active == CmpCameraType::GameStrategy,
                ..default()
            },
            Projection::Perspective(PerspectiveProjection {
                fov: 45.0_f32.to_radians(),
                ..default()
            }),
            CmpTransform2D {
                position: coords.world_center,
                height: 25.0,
                yaw: ANGLE60,
                ..default()
            },
        ))
        .id();

    if settings.active == CmpCameraType::EditorFly {
        cmd.entity(cam_editor_fly).insert(CmpMarkerCameraActive);
    }

    if settings.active == CmpCameraType::EditorTopDownOrthographic {
        cmd.entity(cam_editor_top_down_orthographic)
            .insert(CmpMarkerCameraActive);
    }

    if settings.active == CmpCameraType::GameStrategy {
        cmd.entity(cam_game_top_down).insert(CmpMarkerCameraActive);
    }
}
