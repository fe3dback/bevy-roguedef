#![allow(non_camel_case_types)]

use bevy::prelude::SystemSet;
use strum::{Display, EnumIter};

pub const MAGIC_PREFIX_CONDITION_IN_STATE_IN_GAME_NO_PAUSE: &str = "InGame_NOPAUSE_";
pub const MAGIC_PREFIX_CONDITION_IN_STATE_IN_GAME_ALWAYS: &str = "InGame_ALWAYS_";
pub const MAGIC_MATCH_CONDITION_EDITOR_GIZMOS: &str = "EditorGizmos";
pub const MAGIC_MATCH_CONDITION_EDITOR: &str = "Editor";

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone, Copy, EnumIter, Display)]
pub enum GameSystemSet {
    // note: Order of enum values is matter
    //       systems will run in this order:

    // note: Prefix is important!
    //       use prefix "InGame" - for bind check `in_state(InGame)` to this set

    // For spawning system entities on loading
    LoadingSystem,

    // Editor only logic with game state update
    EditorChangeGameState,

    // -----------------
    // Initialize
    // -----------------
    // Despawn various objects from world
    InGame_NOPAUSE_DespawnObjects,
    // Spawn basic light, sky, etc...
    InGame_NOPAUSE_SpawnWorldEnvironment,
    // Spawn 3d terrain objects
    InGame_NOPAUSE_SpawnWorldTerrain,
    // Spawn player entities, player system objects, etc...
    InGame_NOPAUSE_SpawnPlayerStaff,
    // Spawn enemy mobs
    InGame_NOPAUSE_SpawnMobs,
    // Spawn projectiles/debris/effects/etc...
    InGame_NOPAUSE_SpawnProjectilesAndEffects,

    // -----------------
    // Input
    // -----------------
    InGame_NOPAUSE_ProcessInput,

    // -----------------
    // Gameplay
    // -----------------
    InGame_NOPAUSE_PrepareWeapons,
    InGame_NOPAUSE_UpdateMovements,
    InGame_NOPAUSE_UpdateGameCameras,
    InGame_ALWAYS_UpdateEditorCameras,

    // -----------------
    // Sound effects
    // -----------------
    InGame_NOPAUSE_PlaySound,

    // -----------------
    // Draw
    // -----------------

    // Editor windows
    InGameEditorWindowsDraw,

    // Debug draw gizmos
    InGameEditorGizmosDraw,
}
