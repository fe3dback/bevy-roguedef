mod components;
mod consts;
mod game;
mod plugins;
mod prefabs;

use bevy::app::*;
use bevy::audio::{AudioPlugin, SpatialScale};
use bevy::prelude::{Camera2d, Commands, Window, WindowPlugin};
use bevy::utils::default;
use bevy::window::PresentMode;
use bevy::DefaultPlugins;

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
        // 3-rd plugins
        // game plugins
        .add_systems(Startup, setup)
        .add_plugins(game::plug::Plug {})
        .add_plugins(components::plug::Plug {})
        .add_plugins(plugins::plug::Plug {})
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((Camera2d, CmpMainCamera {}));
}
