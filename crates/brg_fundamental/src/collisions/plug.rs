use bevy::app::{App, Plugin};
use bevy::prelude::{IntoSystemConfigs, Update};
use brg_scene::prelude::{has_editor_feature, EditorFeature, GameSystemSet};

use super::cmp_volume::CmpCollisionVolume;
use super::sys_show_debug_aabb::sys_show_debug_aabb;
use super::sys_show_debug_collisions::show_debug_collisions;

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app
        //
            .register_type::<CmpCollisionVolume>()
            .add_systems(Update, show_debug_collisions.in_set(GameSystemSet::GizmosDraw).run_if(has_editor_feature(EditorFeature::VolumesCollision)))
            .add_systems(Update, sys_show_debug_aabb.in_set(GameSystemSet::GizmosDraw).run_if(has_editor_feature(EditorFeature::VolumesAabb)))
        //-
        ;
    }
}
