mod game;
mod world;

use std::time::Duration;

use bevy::app::*;
use bevy::audio::{AudioPlugin, SpatialScale, Volume};
use bevy::prelude::{
    Camera3d,
    Commands,
    GlobalVolume,
    OrthographicProjection,
    Projection,
    Transform,
    Vec2,
    Vec3,
    Window,
    WindowPlugin,
};
use bevy::render::camera::ScalingMode;
use bevy::utils::default;
use bevy::window::PresentMode;
use bevy::DefaultPlugins;

fn main() {
    // todo: add bevy framepace (+limit fps)
    App::new()
        // std old_plugins
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        name: Some(String::from("bevy roguedef")),
                        present_mode: PresentMode::AutoNoVsync,
                        ..default()
                    }),
                    ..default()
                })
                .set(AudioPlugin {
                    default_spatial_scale: SpatialScale::new(4.0), // todo: setup audio
                    ..default()
                }),
        )
        .insert_resource(GlobalVolume {
            volume: Volume::new(0.01), // todo: set volume
        })
        // 3-rd old_plugins
        .add_plugins(bevy_framepace::FramepacePlugin)
        .insert_resource(bevy_framepace::FramepaceSettings {
            limiter: bevy_framepace::Limiter::Manual(Duration::from_secs_f32(1.0 / 120.0)),
        })
        // game old_plugins
        .add_systems(Startup, setup)
        .add_plugins(brg_core::BrgCorePlugin)
        .add_plugins(brg_editor::BrgEditorPlugin)
        .add_plugins(brg_scene::BrgScenePlugin)
        .add_plugins(game::GamePlugin)
        .run();
}

fn setup(mut commands: Commands) {
    const PIXELS_PER_METER: f32 = 24.0;
    commands.spawn((
        Camera3d::default(),
        Projection::from(Projection::Orthographic(OrthographicProjection {
            near:            0.01,
            far:             1000.0,
            viewport_origin: Vec2::new(0.5, 0.5),
            scaling_mode:    ScalingMode::WindowSize,
            scale:           1.0 / PIXELS_PER_METER,
            area:            Default::default(),
        })),
        Transform {
            translation: Vec3::new(0.0, 0.0, 50.0),
            ..default()
        },
        // CmpTransform2D {
        //     position: V2::splat(0.0),
        //     height: 10.0,
        //     ..default()
        // },
    ));
}
