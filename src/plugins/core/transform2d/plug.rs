use {
    crate::plugins::core::transform2d::sys::transform_apply,
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
            .add_systems(Update, transform_apply);
    }
}
