use bevy::ecs::system::SystemParam;
use bevy::prelude::{info, NextState, Res, ResMut, State};

use crate::prelude::GameState;
use crate::prelude::GameState::Loaded;

#[derive(SystemParam)]
pub struct SupGameManager<'w> {
    pub(crate) state:      Res<'w, State<GameState>>,
    pub(crate) next_state: ResMut<'w, NextState<GameState>>,
}

impl<'w> SupGameManager<'w> {
    #[inline]
    pub fn game_pause(&mut self) {
        self.set_game_pause(true);
    }

    #[inline]
    pub fn game_resume(&mut self) {
        self.set_game_pause(false);
    }

    fn set_game_pause(&mut self, new_pause: bool) {
        match self.state.get() {
            Loaded { game_paused } => {
                if *game_paused == new_pause {
                    return;
                }

                self.next_state.set(Loaded {
                    game_paused: new_pause,
                });
                match new_pause {
                    true => info!("[!] game paused!"),
                    false => info!("[!] game resumed!"),
                }
            }
            _ => return,
        }
    }
}
