use bevy::app::App;
use bevy::prelude::Plugin;

use crate::plugins::gameplay::{integrate_steps, movement, player};

pub struct Plug {}

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app
            //
            .add_plugins(movement::plug::Plug {})
            .add_plugins(player::plug::Plug {})
            .add_plugins(integrate_steps::plug::Plug {});
    }
}
