use {
    bevy::{app::App, prelude::Plugin},
    bevy_inspector_egui::quick::WorldInspectorPlugin,
};

pub struct Plug {}

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.add_plugins(WorldInspectorPlugin::new());
    }
}
