use crate::plugins::core::state::enums::InGame;
use crate::plugins::gameplay::player::sys::spawn_player;
use bevy::app::App;
use bevy::prelude::{OnEnter, Plugin};

pub struct Plug {}

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(InGame), spawn_player)
        ;
    }
}