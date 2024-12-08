use {
    crate::plugins::gameplay::{movement, player},
    bevy::{app::App, prelude::Plugin},
};

pub struct Plug {}

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app
            //
            .add_plugins(movement::plug::Plug {})
            .add_plugins(player::plug::Plug {});
    }
}
