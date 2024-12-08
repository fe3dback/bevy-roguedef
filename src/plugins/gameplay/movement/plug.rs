use {
    crate::plugins::{
        gameplay::movement::sys::{apply_movement, restrict_movement_in_playable_area},
        InGame,
    },
    bevy::{
        app::App,
        prelude::{in_state, IntoSystemConfigs, Plugin, Update},
    },
};

pub struct Plug {}

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app
            //
            .add_systems(Update, apply_movement.run_if(in_state(InGame)))
            .add_systems(
                Update,
                restrict_movement_in_playable_area.run_if(in_state(InGame)),
            );
    }
}
