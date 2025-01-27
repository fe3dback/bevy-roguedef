
use bevy::prelude::{Component, Reflect};
use brg_core::prelude::V2;

#[derive(Component, Debug, Reflect, Default)]
pub struct CmpMarkerCameraActive;

#[derive(Component, Debug, Reflect, Default)]
pub struct CmpCameraAutoFollowSettings {
    pub offset:     V2,
    pub snap_speed: f32,
}
