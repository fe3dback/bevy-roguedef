use bevy::app::{App, Plugin};
use bevy::prelude::{IntoSystemConfigs, Update};
use brg_editor::prelude::{has_editor_feature, EditorFeature};
use brg_scene::prelude::GameSystemSet;

use crate::editor::world_gizmos::sys_debug_draw_coords::sys_debug_draw_world_origin;
use crate::editor::world_gizmos::sys_debug_draw_world_origin::sys_debug_draw_world_mouse_pos;

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app
            //
            .add_systems(Update, sys_debug_draw_world_origin.in_set(GameSystemSet::InGameEditorGizmosDraw).run_if(has_editor_feature(EditorFeature::ShowWorldOriginAxis)))
            .add_systems(Update, sys_debug_draw_world_mouse_pos.in_set(GameSystemSet::InGameEditorGizmosDraw).run_if(has_editor_feature(EditorFeature::ShowWorldMousePosition)))
        //-
        ;
    }
}
