use crate::plugins::core::transform2d::sys::transform_apply;
use bevy::app::App;
use bevy::prelude::{Plugin, Update};

pub struct Plug {}

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, transform_apply)
        ;
    }
}