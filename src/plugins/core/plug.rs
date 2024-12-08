use crate::plugins::core::{state, transform2d};
use bevy::app::App;
use bevy::prelude::Plugin;

pub struct Plug {}

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(state::plug::Plug {})
            .add_plugins(transform2d::plug::Plug {})
        ;
    }
}