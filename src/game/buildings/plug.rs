use bevy::app::{App, Plugin};
use bevy::prelude::Update;

use crate::game::buildings::sys::spawn_building_on_mouse_click;
use crate::game::buildings::CmpBuildingElectricity;

pub struct Plug {}

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app
            //
            //
            .register_type::<CmpBuildingElectricity>()
            // systems
            .add_systems(Update, spawn_building_on_mouse_click)

        //-
        ;
    }
}
