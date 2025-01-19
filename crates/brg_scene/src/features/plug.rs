use bevy::app::App;
use bevy::prelude::Plugin;

use super::enums::SceneType;

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app
            //
            .insert_resource(SceneType::Game)
        //-
        ;
    }
}
