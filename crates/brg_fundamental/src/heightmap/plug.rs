use bevy::app::{App, Plugin};
use bevy::prelude::{IntoSystemConfigs, OnEnter, Update};
use brg_editor::prelude::{has_editor_feature, EditorFeature};
use brg_scene::prelude::{GameSystemSet, InGame};

use super::res::{ResHeightmapCache, ResLandscape};
use super::sys_editor_draw_heightmap::editor_draw_heightmap_around_player;
use super::sys_on_scene_changed_load::sys_on_scene_changed_load_level;

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app
        //
            .register_type::<ResLandscape>()
            .register_type::<ResHeightmapCache>()
            //
            .insert_resource(ResLandscape::default())
            .insert_resource(ResHeightmapCache::default())
            //
            .add_systems(OnEnter(InGame), sys_on_scene_changed_load_level.in_set(GameSystemSet::LoadingSystem))
            .add_systems(Update, editor_draw_heightmap_around_player.in_set(GameSystemSet::InGameEditorGizmosDraw).run_if(has_editor_feature(EditorFeature::LandscapeHeightmap)))
        //-
        ;
    }
}
