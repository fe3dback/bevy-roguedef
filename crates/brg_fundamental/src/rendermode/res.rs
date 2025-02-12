use bevy::prelude::Resource;

use super::enum_mode::ERenderMode;

#[derive(Resource, Default)]
pub struct ResRenderModes {
    pub mode: ERenderMode,
}
