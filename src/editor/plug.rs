use bevy::app::{App, Plugin};

use crate::editor::world_gizmos;

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app
            //
            .add_plugins(world_gizmos::plug::Plug)
        //-
        ;
    }
}
