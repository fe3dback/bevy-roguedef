mod components;
mod prefabs;
mod plugins;

use bevy::app::*;
use bevy::prelude::{Camera2d, Commands};
use bevy::DefaultPlugins;

fn main() {
    App::new()
        // std plugins
        .add_plugins(DefaultPlugins)

        // 3-rd plugins

        // game plugins
        .add_systems(Startup, setup)
        .add_plugins(components::plug::Plug {})
        .add_plugins(plugins::plug::Plug {})
        .run()
    ;
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
