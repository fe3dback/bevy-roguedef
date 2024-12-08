use {
    crate::plugins::core::{state, transform2d},
    bevy::{app::App, prelude::Plugin},
};

pub struct Plug {}

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app
            //
            .add_plugins(state::plug::Plug {})
            .add_plugins(transform2d::plug::Plug {});
    }
}
