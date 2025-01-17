use bevy::app::{App, Plugin};
use bevy::prelude::{OnEnter, PreUpdate};
use brg_scene::prelude::InGame;

use super::sys::init_coord;

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app
        //
            .register_type::<super::res_coords::ResCoords>()
            .insert_resource(super::res_coords::ResCoords::default())
            .add_systems(OnEnter(InGame), init_coord)
            .add_systems(PreUpdate, super::sys::update_world_coords)
        //-
        ;
    }
}
