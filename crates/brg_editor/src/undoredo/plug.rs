use bevy::app::App;
use bevy::prelude::{Plugin, PreUpdate, Update};

use super::res_state::ResEditorCommandState;
use super::sys_on_command_execute::sys_execute_commands;
use super::sys_debug::sys_register_editor_keys;

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app
            //
            .insert_resource(ResEditorCommandState::default())
            //
            .add_systems(PreUpdate, sys_execute_commands)
            .add_systems(Update, sys_register_editor_keys)
        //-
        ;
    }
}
