use bevy::app::{App, Plugin};
use bevy::prelude::{IntoSystemConfigs, OnEnter, OnExit, Update};
use brg_scene::prelude::{GameSystemSet, InGame};

use super::res_state::ResLandscapeState;
use crate::world::landscape::sys_spawn_terrain_root::{
    sys_despawn_terrain_root,
    sys_spawn_initial_chunks,
    sys_spawn_terrain_root,
};

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app
            //
            .insert_resource(ResLandscapeState::default())
            //
            .add_systems(OnEnter(InGame), sys_spawn_terrain_root.in_set(GameSystemSet::InGame_NOPAUSE_SpawnWorldTerrain))
            .add_systems(Update, sys_spawn_initial_chunks.in_set(GameSystemSet::InGame_NOPAUSE_SpawnWorldTerrain))
            .add_systems(OnExit(InGame), sys_despawn_terrain_root.in_set(GameSystemSet::InGame_NOPAUSE_DespawnObjects))
        //-
        ;
    }
}
