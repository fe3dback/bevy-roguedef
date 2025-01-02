use bevy::app::App;
use bevy::prelude::Plugin;

use crate::rand::res::ResRandomSource;

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app
            //
            .insert_resource(ResRandomSource::default())
        //-
        ;
    }
}
