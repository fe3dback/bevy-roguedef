use bevy::app::{App, Plugin};
use bevy::prelude::PreUpdate;

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app
        //
            .register_type::<super::res::ResCoords>()
            .insert_resource(super::res::ResCoords::default())
            .add_systems(PreUpdate, super::sys::update_world_coords)
        //-
        ;
    }
}
