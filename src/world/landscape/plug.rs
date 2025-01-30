use bevy::app::{App, Plugin};
use bevy::prelude::{IntoSystemConfigs, MaterialPlugin, OnEnter, OnExit, Update};
use brg_scene::prelude::{has_feature_in_app, GameSystemSet, Loaded, SceneFeature};

use super::evt_actor_move_in_chunk::EvtActorMoveInChunk;
use super::material::TerrainMaterial;
use super::res_actor_tracker::ResActorTracker;
use super::res_state::ResLandscapeState;
use super::sys_actor_tracker::{
    sys_on_add_tracker_component,
    sys_on_remove_tracker_component,
    sys_track_actors,
};
use super::sys_spawn_terrain_root::{
    sys_despawn_terrain_root,
    sys_spawn_chunks_on_actor_moves,
    sys_spawn_initial_chunks,
    sys_spawn_terrain_root,
};

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(ResLandscapeState::default())
            .insert_resource(ResActorTracker::default())
            .add_event::<EvtActorMoveInChunk>()
        //-
        ;

        if has_feature_in_app(app, SceneFeature::WorldLandscape) {
            app
                //
                .add_plugins(MaterialPlugin::<TerrainMaterial>::default())
                //
                .add_systems(OnEnter(Loaded), sys_spawn_terrain_root.in_set(GameSystemSet::SpawnWorldTerrain))
                .add_systems(Update, (
                    sys_spawn_initial_chunks,
                    sys_spawn_chunks_on_actor_moves,
                ).in_set(GameSystemSet::SpawnWorldTerrain))
                .add_systems(Update, sys_track_actors.in_set(GameSystemSet::NOT_ON_PAUSE__UpdateGameplayCaches))
                .add_systems(OnExit(Loaded), sys_despawn_terrain_root.in_set(GameSystemSet::NOT_ON_PAUSE__DespawnObjects))
                //
                .add_observer(sys_on_add_tracker_component)
                .add_observer(sys_on_remove_tracker_component)
            //-
            ;
        }
    }
}
