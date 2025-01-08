use bevy::app::{App, Plugin};
use bevy::prelude::{IntoSystemConfigs, OnEnter, Update};
use brg_scene::prelude::{has_feature, GameSystemSet, InGame, SceneFeature};

use crate::world::camera;
use crate::world::sys_spawn_example_objects::{
    debug_mark_terrain_as_heightmap_source,
    spawn_example_objects,
};
use crate::world::sys_spawn_light::spawn_light;

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app
            //
            .add_plugins(camera::plug::Plug)
            .add_systems(OnEnter(InGame), spawn_light.in_set(GameSystemSet::InGame_NOPAUSE_SpawnWorldEnvironment).run_if(has_feature(SceneFeature::WorldEnvLight)))
            .add_systems(OnEnter(InGame), (spawn_example_objects).in_set(GameSystemSet::InGame_NOPAUSE_SpawnWorldTerrain).run_if(has_feature(SceneFeature::ExampleCubes)))
            .add_systems(Update, debug_mark_terrain_as_heightmap_source) // todo: delete
            // .add_systems(Update, debug_tmp_update_heightmap_from_terrain) // todo: delete
        //-
        ;
    }
}
