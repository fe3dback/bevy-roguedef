use bevy::app::{App, Plugin};
use bevy::prelude::{IntoSystemConfigs, OnEnter, Update};
use brg_editor::prelude::{has_editor_feature, EditorFeature};
use brg_scene::prelude::{has_feature, GameSystemSet, InGame, SceneFeature};

use crate::world::camera;
use crate::world::sys_debug_draw_world_origin::sys_debug_draw_world_origin;
use crate::world::sys_spawn_example_objects::spawn_example_objects;
use crate::world::sys_spawn_light::spawn_light;

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app
            //
            .add_plugins(camera::plug::Plug)
            .add_systems(OnEnter(InGame), spawn_light.in_set(GameSystemSet::InGameSpawnWorldEnvironment).run_if(has_feature(SceneFeature::WorldEnvLight)))
            .add_systems(OnEnter(InGame), spawn_example_objects.in_set(GameSystemSet::InGameSpawnWorldTerrain).run_if(has_feature(SceneFeature::ExampleCubes)))
            .add_systems(Update, sys_debug_draw_world_origin.in_set(GameSystemSet::InGameEditorGizmosDraw).run_if(has_feature(SceneFeature::EditorGizmos)).run_if(has_editor_feature(EditorFeature::ShowWorldOriginAxis)))
        //-
        ;
    }
}
