use crate::plugins::core::state::enums::GameState;
use bevy::app::App;
use bevy::prelude::{AppExtStates, Plugin};

pub struct Plug {}

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.
            init_state::<GameState>()
        ;
    }
}