mod components;
mod consts;
mod game;
mod plugins;
mod prefabs;

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
use bevy_sprite3d::Sprite3dPlugin;
use bevy_vector_shapes::Shape2dPlugin;

use crate::game::common::CmpMainCamera;

/// Spatial audio uses the distance to attenuate the sound volume. In 2D with the default camera,
/// 1 pixel is 1 unit of distance, so we use a scale
/// 1m = 200px
const AUDIO_SCALE: f32 = 1. / 200.0;

fn main() {
    // todo: add bevy framepace (+limit fps)
    App::new()
        // std plugins
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
                    default_spatial_scale: SpatialScale::new_2d(AUDIO_SCALE),
                    ..default()
                }),
        )
        .insert_resource(GlobalVolume {
            volume: Volume::new(0.01), // todo: set volume
        })
        // 3-rd plugins
        .add_plugins(bevy_framepace::FramepacePlugin)
        .insert_resource(bevy_framepace::FramepaceSettings {
            limiter: bevy_framepace::Limiter::Manual(Duration::from_secs_f32(1.0 / 120.0)),
        })
        .add_plugins(Shape2dPlugin::default())
        .add_plugins(Sprite3dPlugin {})
        // game plugins
        .add_systems(Startup, setup)
        .add_plugins(game::plug::Plug {})
        .add_plugins(components::plug::Plug {})
        .add_plugins(plugins::plug::Plug {})
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
        CmpMainCamera {},
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
