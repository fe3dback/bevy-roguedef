use bevy::app::App;
use bevy::prelude::{in_state, IntoSystemConfigs, Plugin, Update};

use crate::old_plugins::gameplay::movement::sys::{
    apply_movement,
    restrict_movement_in_playable_area,
};
use crate::old_plugins::InGame;

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
