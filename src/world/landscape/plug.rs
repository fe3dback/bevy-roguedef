use bevy::app::{App, Plugin};
use bevy::prelude::{IntoSystemConfigs, MaterialPlugin, OnEnter, OnExit, Update};
use brg_scene::prelude::{
    has_editor_feature,
    has_feature_in_app,
    EditorFeature,
    GameSystemSet,
    Loaded,
    SceneFeature,
};

use super::evt_actor_move_in_chunk::EvtLodPoeMovedIntoNewChunk;
use super::material::TerrainMaterial;
use super::res_state::ResLandscapeState;
use super::sys_debug_quad_tree::sys_debug_quad_tree;
use super::sys_spawn_terrain_root::{
    sys_despawn_terrain_root,
    sys_respawn_chunks_when_lod_poe_changed,
    sys_spawn_terrain_root,
};
use super::sys_update_lod_poe::sys_update_lod_poe;

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(ResLandscapeState::default())
            .add_event::<EvtLodPoeMovedIntoNewChunk>()
        //-
        ;

        if has_feature_in_app(app, SceneFeature::WorldLandscape) {
            app
                //
                .add_plugins(MaterialPlugin::<TerrainMaterial>::default())
                //
                .add_systems(OnEnter(Loaded), sys_spawn_terrain_root.in_set(GameSystemSet::SpawnWorldTerrain))
                .add_systems(Update, sys_respawn_chunks_when_lod_poe_changed.in_set(GameSystemSet::SpawnWorldTerrain))
                .add_systems(Update, sys_update_lod_poe.in_set(GameSystemSet::NOT_ON_PAUSE__UpdateGameplayCaches))
                .add_systems(Update, sys_debug_quad_tree.in_set(GameSystemSet::GizmosDraw).run_if(has_editor_feature(EditorFeature::LandscapeHeightmap)))
                .add_systems(OnExit(Loaded), sys_despawn_terrain_root.in_set(GameSystemSet::NOT_ON_PAUSE__DespawnObjects))
            //-
            ;
        }
    }
}
