use std::cell::RefCell;

use bevy::ecs::system::SystemParam;
use bevy::prelude::{error, Bundle, ResMut};

use super::command::EditorCommand;
use super::command_cmn_spawn::EditorCommandSpawn;
use super::res_state::ResEditorCommandState;

#[derive(SystemParam)]
pub struct SupEditorCommands<'w> {
    state: ResMut<'w, ResEditorCommandState>,
}

impl<'w, 's> SupEditorCommands<'w> {
    pub fn apply(&mut self, command: Box<dyn EditorCommand + Send + Sync + 'static>) {
        let elem = Some(RefCell::new(command));
        let mut stack = self.state.stack.lock().unwrap();

        match stack.write(elem) {
            Err(e) => error!("cannot apply to undo-redo stack: {}", e),
            _ => {}
        }
    }

    pub fn apply_spawn<T: Bundle>(
        &mut self,
        name: String,
        factory: impl Fn() -> T + Send + Sync + 'static,
    ) {
        self.apply(Box::new(EditorCommandSpawn::new(name, factory)));
    }

    pub fn undo(&mut self) {
        match self.state.required_undo.lock() {
            Ok(mut cnt) => *cnt = *cnt + 1,
            _ => {}
        };
    }

    pub fn redo(&mut self) {
        match self.state.required_redo.lock() {
            Ok(mut cnt) => *cnt = *cnt + 1,
            _ => {}
        };
    }
}
