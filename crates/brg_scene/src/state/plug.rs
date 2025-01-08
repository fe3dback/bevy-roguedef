use bevy::app::App;
use bevy::prelude::{AppExtStates, IntoSystemConfigs, Plugin, Update};

use crate::prelude::{GameState, GameSystemSet, InGame};
use crate::state::sys_switch_pause::editor_switch_pause;

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app
            //
            .add_computed_state::<InGame>()
            .init_state::<GameState>()
            .enable_state_scoped_entities::<GameState>()
            .add_systems(
                Update,
                editor_switch_pause.in_set(GameSystemSet::EditorChangeGameState),
            );
    }
}
