mod editor;
mod game;
mod prefabs;
mod units;
mod world;

use std::time::Duration;

use bevy::app::*;
use bevy::audio::{AudioPlugin, SpatialScale, Volume};
use bevy::prelude::{GlobalVolume, Window, WindowPlugin};
use bevy::utils::default;
use bevy::window::PresentMode;
use bevy::DefaultPlugins;

fn main() {
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
                    default_spatial_scale: SpatialScale::new(1.0),
                    global_volume:         GlobalVolume {
                        volume: Volume::new(1.0), // todo: set volume
                    },
                }),
        )
        // 3-rd old_plugins
        .add_plugins(bevy_framepace::FramepacePlugin)
        .insert_resource(bevy_framepace::FramepaceSettings {
            limiter: bevy_framepace::Limiter::Manual(Duration::from_secs_f32(1.0 / 120.0)),
        })
        // game old_plugins
        .add_plugins(brg_core::BrgCorePlugin)
        .add_plugins(brg_editor::BrgEditorPlugin)
        .add_plugins(brg_scene::BrgScenePlugin)
        .add_plugins(brg_fundamental::BrgFundamentalPlugin)
        .add_plugins(game::GamePlugin)
        .run();
}
