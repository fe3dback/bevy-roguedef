use bevy::app::App;
use bevy::prelude::Plugin;

use crate::old_plugins;

pub struct Plug {}

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app
            //
            .add_plugins(old_plugins::core::plug::Plug {})
            .add_plugins(old_plugins::assets::plug::Plug {})
            .add_plugins(old_plugins::gameplay::plug::Plug {});
    }
}
