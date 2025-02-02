use bevy::app::{App, Plugin};
use bevy::prelude::{IntoSystemConfigs, OnEnter, Update};
use brg_scene::prelude::{has_editor_feature, EditorFeature, GameSystemSet, Loaded};

use super::res::ResLandscape;
use super::sys_editor_draw_heightmap::editor_draw_heightmap_around_player;
use super::sys_on_scene_changed_load::sys_on_scene_changed_load_level;

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app
        //
            .register_type::<ResLandscape>()
            //
            .insert_resource(ResLandscape::default())
            //
            .add_systems(OnEnter(Loaded), sys_on_scene_changed_load_level.in_set(GameSystemSet::ALLOW_ON_LOAD__LoadingSystem))
            .add_systems(Update, editor_draw_heightmap_around_player.in_set(GameSystemSet::GizmosDraw).run_if(has_editor_feature(EditorFeature::LandscapeHeightmap)))
        //-
        ;
    }
}
