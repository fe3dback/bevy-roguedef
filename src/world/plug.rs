use bevy::app::{App, Plugin};
use bevy::prelude::{IntoSystemConfigs, OnEnter};
use brg_scene::prelude::{has_feature, GameSystemSet, Loaded, SceneFeature};

use super::sys_spawn_example_objects::spawn_example_objects;
use super::sys_spawn_light::spawn_light;
use super::{camera, landscape};

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app
            //
            .add_plugins(camera::plug::Plug)
            .add_plugins(landscape::plug::Plug)
            .add_systems(OnEnter(Loaded), spawn_light.in_set(GameSystemSet::SpawnWorldEnvironment).run_if(has_feature(SceneFeature::WorldEnvLight)))
            .add_systems(OnEnter(Loaded), (spawn_example_objects).in_set(GameSystemSet::SpawnWorldTerrain).run_if(has_feature(SceneFeature::ExampleCubes)))
        //-
        ;
    }
}
