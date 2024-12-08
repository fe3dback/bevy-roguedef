use crate::plugins::core::state::enums::InGame;
use crate::plugins::gameplay::player::sys::{spawn_player, wasd_movement};
use bevy::app::App;
use bevy::prelude::{in_state, IntoSystemConfigs, OnEnter, Plugin, Update};

pub struct Plug {}

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(InGame), spawn_player)
            .add_systems(Update, wasd_movement.run_if(in_state(InGame)))
        ;
    }
}