use bevy::input::ButtonInput;
use bevy::prelude::{Camera, Commands, DetectChanges, Entity, KeyCode, Query, Res, ResMut};
use brg_scene::prelude::{SceneFeature, SupFeatures};

use super::cmp::CmpMarkerCameraActive;
use super::enums::CmpCameraType;
use super::res::ResCameraSettings;

pub fn switch_camera_on_keyboard_input(
    kbr: Res<ButtonInput<KeyCode>>,
    mut settings: ResMut<ResCameraSettings>,
    features: SupFeatures,
) {
    if !kbr.just_pressed(KeyCode::Backquote) {
        return;
    }

    {
        if features.has_feature(SceneFeature::Units) {
            settings.active = match settings.active {
                CmpCameraType::EditorFly => CmpCameraType::EditorTopDownOrthographic,
                CmpCameraType::EditorTopDownOrthographic => CmpCameraType::GameStrategy,
                CmpCameraType::GameStrategy => CmpCameraType::EditorFly,
                _ => CmpCameraType::EditorFly,
            };
        } else {
            settings.active = match settings.active {
                CmpCameraType::EditorFly => CmpCameraType::EditorTopDownOrthographic,
                CmpCameraType::EditorTopDownOrthographic => CmpCameraType::EditorFly,
                _ => CmpCameraType::EditorFly,
            };
        }
    }

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
