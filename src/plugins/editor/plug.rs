use {
    crate::plugins::editor::{grid, inspector, stats},
    bevy::{app::App, prelude::Plugin},
};

pub struct Plug {}

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app
            //
            .add_plugins(stats::plug::Plug {})
            .add_plugins(inspector::plug::Plug {})
            .add_plugins(grid::plug::Plug {});
    }
}
