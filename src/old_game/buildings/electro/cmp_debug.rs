use bevy::prelude::{Color, Component, Reflect};
use bevy_inspector_egui::InspectorOptions;

#[derive(Component, Debug, Reflect, Default, InspectorOptions)]
pub struct CmpDebugElectricityOutline {
    pub on:    bool,
    pub color: Color,
    pub time:  String,
}
