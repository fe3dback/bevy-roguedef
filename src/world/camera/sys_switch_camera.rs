use bevy::input::ButtonInput;
use bevy::prelude::{Camera, Commands, DetectChanges, Entity, KeyCode, Query, Res, ResMut};

use crate::world::camera::cmp::CmpMarkerCameraActive;
use crate::world::camera::enums::CmpCameraType;
use crate::world::camera::res::ResCameraSettings;

pub fn switch_camera_on_keyboard_input(
    kbr: Res<ButtonInput<KeyCode>>,
    mut settings: ResMut<ResCameraSettings>,
) {
    if !kbr.just_pressed(KeyCode::Backquote) {
        return;
    }

    // todo: move from camera (this is global game state)

    settings.active = match settings.active {
        CmpCameraType::EditorFly => CmpCameraType::GameTopDown,
        CmpCameraType::GameTopDown => CmpCameraType::EditorFly,
    };

    if settings.active != CmpCameraType::EditorFly {
        settings.editor_fly_grab_active = false;
    }
}

pub fn switch_camera_on_settings_change(
    settings: Res<ResCameraSettings>,
    mut query: Query<(Entity, &mut Camera, &CmpCameraType)>,
    mut cmd: Commands,
) {
    if !settings.is_changed() {
        return;
    }

    for (ent, mut cam, cam_type) in &mut query {
        cam.is_active = cam_type == &settings.active;

        if cam.is_active {
            cmd.entity(ent).insert(CmpMarkerCameraActive);
        } else {
            cmd.entity(ent).remove::<CmpMarkerCameraActive>();
        }
    }
}
