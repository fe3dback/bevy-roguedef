use crate::game::buildings::sys::spawn_building_on_mouse_click;
use bevy::prelude::Update;
use {
    crate::game::buildings::CmpBuilding,
    bevy::app::{App, Plugin}
    ,
};

pub struct Plug {}

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app
            //
            //
            .register_type::<CmpBuilding>()
            // systems
            .add_systems(Update, spawn_building_on_mouse_click)

        //-
        ;
    }
}
