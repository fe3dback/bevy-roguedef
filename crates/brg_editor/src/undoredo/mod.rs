mod command;
mod command_cmn_spawn;
pub mod plug;
mod res_state;
mod stack;
mod sup_editor_commands;
mod sys_debug;
mod sys_on_command_execute;

pub mod prelude {
    pub use super::command::EditorCommand;
}
