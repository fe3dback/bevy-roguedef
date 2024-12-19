use {bevy::prelude::Reflect, bevy_inspector_egui::InspectorOptions};

#[derive(Reflect, InspectorOptions, Debug, Default, Clone, Copy)]
pub struct ChannelState {
    pub capacity: f32,
    pub charge: f32,
    pub throughput_in: f32,
    pub throughput_out: f32,
    pub throughput_max_in: f32,
    pub throughput_max_out: f32,
}
