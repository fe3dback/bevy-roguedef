use bevy::app::{App, Plugin};

use crate::{units, world};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
        //
        .add_plugins(world::plug::Plug)
        .add_plugins(units::plug::Plug)
        //-
        ;
    }
}
