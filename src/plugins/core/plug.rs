use bevy::app::App;
use bevy::prelude::Plugin;

use crate::plugins::core::state;

pub struct Plug {}

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app
            //
            .add_plugins(state::plug::Plug {});
    }
}
