use bevy::app::{App, Plugin};
use bevy::prelude::{Startup, Update};

use crate::game::buildings::sys::{
    load_ldtk_circuit,
    spawn_building_on_mouse_click,
    spawn_starting_buildings,
    ResLdtkHandles,
};
use crate::game::buildings::CmpBuildingElectricity;

pub struct Plug {}

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app
            //
            //
            .register_type::<CmpBuildingElectricity>()
            .insert_resource(ResLdtkHandles::default())
            // systems
            .add_systems(Startup, load_ldtk_circuit)
            .add_systems(Update, spawn_starting_buildings)
            .add_systems(Update, spawn_building_on_mouse_click)

        //-
        ;
    }
}
