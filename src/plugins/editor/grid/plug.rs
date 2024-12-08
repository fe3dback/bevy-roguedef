use {
    crate::plugins::editor::grid::sys::draw_grid,
    bevy::{
        app::App,
        prelude::{Plugin, Update},
    },
};

pub struct Plug {}

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app
            //
            .add_systems(Update, draw_grid);
    }
}
