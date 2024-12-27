use bevy::app::{App, Plugin};

use crate::world;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
        //
        .add_plugins(world::plug::Plug)
        //-
        ;
    }
}
