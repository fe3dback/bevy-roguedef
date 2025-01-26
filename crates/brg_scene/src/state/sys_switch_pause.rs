use bevy::input::ButtonInput;
use bevy::prelude::{KeyCode, Res, State};

use super::sup_game_manager::SupGameManager;
use crate::prelude::GameState;

pub fn editor_switch_pause(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut manager: SupGameManager,
    state: Res<State<GameState>>,
) {
    if !keyboard.just_pressed(KeyCode::F1) {
        return;
    }

    match state.get() {
        GameState::Loaded { game_paused } => match game_paused {
            true => manager.game_resume(),
            false => manager.game_pause(),
        },
        _ => {}
    }
}
