use bevy::app::App;
use bevy::prelude::Plugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub struct Plug {}

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(WorldInspectorPlugin::new())
        ;
    }
}