use bevy::app::{App, Plugin};
use bevy::prelude::{IntoSystemConfigs, Update};
use brg_editor::prelude::{has_editor_feature, EditorFeature};
use brg_scene::prelude::GameSystemSet;

use super::cmp_volume::CmpCollisionVolume;
use super::sys_show_debug_collisions::show_debug_collisions;

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app
        //
            .register_type::<CmpCollisionVolume>()
            .add_systems(Update, show_debug_collisions.in_set(GameSystemSet::InGameEditorGizmosDraw).run_if(has_editor_feature(EditorFeature::ShowCollisionVolumes)))
        //-
        ;
    }
}
