use bevy::app::App;
use bevy::prelude::Plugin;

use super::enums::DEFAULT_SCENE;

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app
            //
            .insert_resource(DEFAULT_SCENE)
        //-
        ;
    }
}
