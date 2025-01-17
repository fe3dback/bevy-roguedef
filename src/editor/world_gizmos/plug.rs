use bevy::app::{App, Plugin};
use bevy::prelude::{IntoSystemConfigs, Update};
use brg_editor::prelude::{has_editor_feature, EditorFeature};
use brg_scene::prelude::GameSystemSet;

use super::sys_debug_draw_coords::sys_debug_draw_world_origin;
use super::sys_debug_draw_world_origin::sys_debug_draw_world_mouse_pos;

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app
            //
            .add_systems(Update, sys_debug_draw_world_origin.in_set(GameSystemSet::InGameEditorGizmosDraw).run_if(has_editor_feature(EditorFeature::GizmosOriginAxis)))
            .add_systems(Update, sys_debug_draw_world_mouse_pos.in_set(GameSystemSet::InGameEditorGizmosDraw).run_if(has_editor_feature(EditorFeature::GizmosWorldMouse)))
        //-
        ;
    }
}
