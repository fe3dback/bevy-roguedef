use {
    crate::plugins::{gameplay::movement::sys::apply_movement, InGame},
    bevy::{
        app::App,
        prelude::{in_state, IntoSystemConfigs, Plugin, Update},
    },
};

pub struct Plug {}

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, apply_movement.run_if(in_state(InGame)));
    }
}
