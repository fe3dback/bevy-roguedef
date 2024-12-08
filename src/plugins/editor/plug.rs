use crate::plugins::editor::{inspector, stats};
use bevy::app::App;
use bevy::prelude::Plugin;

pub struct Plug {}

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(stats::plug::Plug {})
            .add_plugins(inspector::plug::Plug {})
        ;
    }
}