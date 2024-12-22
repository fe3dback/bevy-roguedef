use bevy::app::{App, Plugin};
use bevy::prelude::{OnEnter, Startup, Update};

use crate::game::buildings::electro;
use crate::game::buildings::sys::{
    choose_debug_building_to_spawn,
    load_ldtk_circuit,
    spawn_building_on_mouse_click,
    spawn_starting_buildings,
    ResBuildingDebugChoose,
    ResLdtkHandles,
};
use crate::plugins::InGame;

pub struct Plug {}

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app
            //
            .add_plugins(electro::plug::Plug {})
            //
            .insert_resource(ResLdtkHandles::default())
            .insert_resource(ResBuildingDebugChoose::default())
            // systems
            .add_systems(Startup, load_ldtk_circuit)
            .add_systems(OnEnter(InGame), spawn_starting_buildings)
            .add_systems(Update, (choose_debug_building_to_spawn, spawn_building_on_mouse_click))

        //-
        ;
    }
}
