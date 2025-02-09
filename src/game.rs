use bevy::app::{App, Plugin};
use brg_core::prelude::clear_debug_log_file;

use super::editor;
use super::units;
use super::world;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        clear_debug_log_file();

        app
        //
        .add_plugins(editor::plug::Plug)
        .add_plugins(world::plug::Plug)
        .add_plugins(units::plug::Plug)
        //-
        ;
    }
}
