use bevy::app::{App, Plugin};
use bevy::prelude::Update;

use crate::heightmap::sys_debug_draw::debug_draw_grid;
use crate::prelude::ResHeightmap;

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app
        //
            .register_type::<ResHeightmap>()
            .insert_resource(ResHeightmap::default())
            .add_systems(Update, debug_draw_grid) // todo: delete me
        //-
        ;
    }
}
