use bevy::app::{App, Plugin};
use bevy::prelude::Update;

use super::sys_tick;
use crate::game::buildings::electro::{cmp, cmp_debug, evt, internal, res_graph, sys_debug_draw};

pub struct Plug {}

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app
            //
            .register_type::<res_graph::ResBuildingWorldGraphs>()
            .register_type::<cmp::CmpBuildingElectricity>()
            .register_type::<internal::graph::Graph>()
            .register_type::<internal::tree::TreeRoot>()
            .register_type::<internal::tree::TreeLeaf>()
            .insert_resource(res_graph::ResBuildingWorldGraphs::default())
            .add_event::<evt::EvtOnBuildingChargeChanged>()
            .add_systems(Update, sys_tick::electricity_tick)
            // debug
            .register_type::<cmp_debug::CmpDebugElectricityOutline>()
            // debug draw
            .add_systems(Update, sys_debug_draw::draw_graph_tree)
            .add_systems(Update, sys_debug_draw::debug_outline_reset)
            .add_systems(Update, sys_debug_draw::debug_outline_show);
    }
}
