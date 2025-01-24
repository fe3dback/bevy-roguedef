use bevy::input::ButtonInput;
use bevy::prelude::{KeyCode, Res};

use super::sup_editor_commands::SupEditorCommands;

pub fn sys_register_editor_keys(
    mut x: SupEditorCommands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.pressed(KeyCode::ControlLeft)
        && keyboard_input.pressed(KeyCode::ShiftLeft)
        && keyboard_input.just_pressed(KeyCode::KeyZ)
    {
        x.redo();
        return;
    }

    if keyboard_input.pressed(KeyCode::ControlLeft) && keyboard_input.just_pressed(KeyCode::KeyZ) {
        x.undo();
        return;
    }
}
