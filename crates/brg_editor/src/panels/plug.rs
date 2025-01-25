use bevy::app::App;
use bevy::prelude::Plugin;

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app
            // 
            .add_plugins(super::doodads::plug::Plug)
        //-
        ;
    }
}
