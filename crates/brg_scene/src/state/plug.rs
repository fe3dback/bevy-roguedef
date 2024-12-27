use bevy::app::App;
use bevy::prelude::{AppExtStates, Plugin};

use crate::prelude::{GameState, InGame};

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app
            //
            .add_computed_state::<InGame>()
            .init_state::<GameState>()
            .enable_state_scoped_entities::<GameState>();
    }
}
