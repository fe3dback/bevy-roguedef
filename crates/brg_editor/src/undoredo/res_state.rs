use std::cell::RefCell;
use std::sync::Mutex;

use bevy::prelude::Resource;

use super::command::EditorCommandSync;
use super::stack::Stack;

#[derive(Resource)]
pub struct ResEditorCommandState {
    pub(super) stack:         Mutex<Stack<Option<RefCell<Box<EditorCommandSync>>>>>,
    /// count of undo button is pressed
    pub(super) required_redo: Mutex<u32>,
    /// count of redo button is pressed
    pub(super) required_undo: Mutex<u32>,
}

impl Default for ResEditorCommandState {
    fn default() -> Self {
        Self {
            stack:         Mutex::new(Stack::new(256)),
            required_redo: Mutex::new(0),
            required_undo: Mutex::new(0),
        }
    }
}
