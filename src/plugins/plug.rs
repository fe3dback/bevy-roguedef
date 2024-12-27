use bevy::app::App;
use bevy::prelude::Plugin;

use crate::plugins;

pub struct Plug {}

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app
            //
            .add_plugins(plugins::core::plug::Plug {})
            .add_plugins(plugins::assets::plug::Plug {})
            .add_plugins(plugins::gameplay::plug::Plug {});
    }
}
