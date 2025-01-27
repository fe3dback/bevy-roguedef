use bevy::prelude::{Reflect, ReflectResource, Resource};
use brg_core::prelude::V2;

use super::enums::CmpCameraType;

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct ResCameraSettings {
    pub active:                      CmpCameraType,
    pub editor_fly_grab_active:      bool,
    pub editor_fly_look_sensitivity: V2,
    pub editor_fly_speed_mul:        f32,
    pub editor_topdown_mouse_scroll: f32, // 0 - 1000.0
}

impl Default for ResCameraSettings {
    fn default() -> Self {
        Self {
            active:                      CmpCameraType::default(),
            editor_fly_grab_active:      false,
            editor_fly_look_sensitivity: V2::new(0.5, 0.5),
            editor_fly_speed_mul:        1.0,
            editor_topdown_mouse_scroll: 500.0,
        }
    }
}
