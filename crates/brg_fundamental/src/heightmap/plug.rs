use bevy::app::{App, Plugin};
use bevy::prelude::{IntoSystemConfigs, Update};
use brg_editor::prelude::{has_editor_feature, EditorFeature};
use brg_scene::prelude::GameSystemSet;

use crate::heightmap::sys_editor_draw_heightmap::editor_draw_heightmap_around_player;
use crate::heightmap::sys_import_heightmap_from_entities::import_heightmap_from_entities;
use crate::prelude::ResHeightmap;

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app
        //
            .register_type::<ResHeightmap>()
            .insert_resource(ResHeightmap::default())
            .add_systems(Update, editor_draw_heightmap_around_player.in_set(GameSystemSet::InGameEditorGizmosDraw).run_if(has_editor_feature(EditorFeature::DrawHeightmapPoints)))
            .add_systems(Update, import_heightmap_from_entities)
        //-
        ;
    }
}
