mod editor;
mod game;
mod prefabs;
mod units;
mod world;

use std::time::Duration;

use bevy::app::*;
use bevy::audio::{AudioPlugin, SpatialScale, Volume};
use bevy::color::palettes::basic::WHITE;
use bevy::image::{ImageAddressMode, ImageFilterMode, ImageSamplerDescriptor};
use bevy::pbr::wireframe::{WireframeConfig, WireframePlugin};
use bevy::prelude::{ClearColor, Color, GlobalVolume, ImagePlugin, Window, WindowPlugin};
use bevy::render::settings::{RenderCreation, WgpuFeatures, WgpuSettings};
use bevy::render::RenderPlugin;
use bevy::utils::default;
use bevy::window::PresentMode;
use bevy::DefaultPlugins;

fn main() {
    App::new()
        // std old_plugins
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin {
                    default_sampler: ImageSamplerDescriptor {
                        min_filter: ImageFilterMode::Linear,
                        mag_filter: ImageFilterMode::Linear,
                        mipmap_filter: ImageFilterMode::Linear,
                        address_mode_u: ImageAddressMode::Repeat,
                        address_mode_v: ImageAddressMode::Repeat,
                        address_mode_w: ImageAddressMode::Repeat,
                        ..default()
                    },
                })
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
                        volume: Volume::new(0.1), // todo: set volume
                    },
                })
                .set(RenderPlugin {
                    render_creation: RenderCreation::Automatic(WgpuSettings {
                        features: WgpuFeatures::POLYGON_MODE_LINE,
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins(WireframePlugin)
        // Wireframes can be configured with this resource. This can be changed at runtime.
        .insert_resource(WireframeConfig {
            global:        false,
            default_color: WHITE.into(),
        })
        .insert_resource(ClearColor(Color::srgb(0.01, 0.02, 0.01)))
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
