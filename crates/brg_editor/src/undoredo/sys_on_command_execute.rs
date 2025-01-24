use bevy::ecs::system::SystemState;
use bevy::prelude::{error, info, Res, World};

use super::res_state::ResEditorCommandState;

pub fn sys_execute_commands(w: &mut World, params: &mut SystemState<(Res<ResEditorCommandState>)>) {
    let state = params.get(w);
    let unsafe_world = w.as_unsafe_world_cell_readonly();

    let mut stack = match state.stack.lock() {
        Ok(x) => x,
        Err(_) => return,
    };

    // execute
    {
        for cmd in stack.drain_not_executed() {
            let Some(cmd) = cmd else {
                continue;
            };

            let mut cmd = cmd.borrow_mut();

            // SAFETY: not so safe, but can be broken only we change ResEditorCommandState in this world
            match cmd.apply(unsafe { unsafe_world.world_mut() }) {
                Err(err) => error!("Failed to apply command: {}", err),
                _ => {}
            }
        }
    }

    // undo
    {
        let len = match state.required_undo.lock() {
            Ok(mut value) => {
                let len = *value;
                *value = 0;

                len
            }
            _ => 0,
        };

        for _ in 0..len {
            let Ok(cmd) = stack.undo() else {
                break;
            };

            let Some(cmd) = cmd else {
                continue;
            };

            let mut cmd = cmd.borrow_mut();

            // SAFETY: not so safe, but can be broken only we change ResEditorCommandState in this world
            match cmd.compensate(unsafe { unsafe_world.world_mut() }) {
                Err(err) => error!("Failed to compensate command: {}", err),
                _ => {}
            }
        }
    }

    // redo
    {
        let len = match state.required_redo.lock() {
            Ok(mut value) => {
                let len = *value;
                *value = 0;

                len
            }
            _ => 0,
        };

        for _ in 0..len {
            let Ok(cmd) = stack.redo() else {
                break;
            };

            let Some(cmd) = cmd else {
                continue;
            };

            let mut cmd = cmd.borrow_mut();

            // SAFETY: not so safe, but can be broken only we change ResEditorCommandState in this world
            match cmd.apply(unsafe { unsafe_world.world_mut() }) {
                Err(err) => error!("Failed to compensate command: {}", err),
                _ => {}
            }
        }
    }
}
