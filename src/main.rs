mod components;
mod consts;
mod game;
mod plugins;
mod prefabs;

use bevy::{
    app::*,
    prelude::{Camera2d, Commands, Window, WindowPlugin},
    utils::default,
    window::PresentMode,
    DefaultPlugins,
};

fn main() {
    // todo: add bevy framepace (+limit fps)
    App::new()
        // std plugins
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                name: Some(String::from("bevy roguedef")),
                present_mode: PresentMode::AutoNoVsync,
                ..default()
            }),
            ..default()
        }))
        // 3-rd plugins
        // game plugins
        .add_systems(Startup, setup)
        .add_plugins(game::plug::Plug {})
        .add_plugins(components::plug::Plug {})
        .add_plugins(plugins::plug::Plug {})
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
