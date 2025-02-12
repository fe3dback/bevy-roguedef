use bevy::pbr::wireframe::WireframeConfig;
use bevy::prelude::{ButtonInput, DetectChanges, KeyCode, Res, ResMut};

use super::enum_mode::ERenderMode;
use super::res::ResRenderModes;

pub fn sys_switch_mode_on_keyboard(
    kb: Res<ButtonInput<KeyCode>>,
    mut state: ResMut<ResRenderModes>,
) {
    if kb.just_pressed(KeyCode::F2) {
        state.mode = match state.mode {
            ERenderMode::Lit => ERenderMode::Wireframe,
            ERenderMode::Wireframe => ERenderMode::Lit,
        }
    }
}

pub fn sys_on_mode_switched(state: Res<ResRenderModes>, mut wireframe: ResMut<WireframeConfig>) {
    if !state.is_changed() {
        return;
    }

    match state.mode {
        ERenderMode::Lit => {
            wireframe.global = false;
        }
        ERenderMode::Wireframe => {
            wireframe.global = true;
        }
    }
}
