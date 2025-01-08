use bevy::prelude::SystemSet;
use strum::{Display, EnumIter};

pub const MAGIC_PREFIX_CONDITION_IN_STATE_IN_GAME: &str = "InGame";
pub const MAGIC_MATCH_CONDITION_EDITOR_GIZMOS: &str = "EditorGizmos";

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone, Copy, EnumIter, Display)]
pub enum GameSystemSet {
    // note: Order of enum values is matter
    //       systems will run in this order:

    // note: Prefix is important!
    //       use prefix "InGame" - for bind check `in_state(InGame)` to this set

    // For spawning system entities on loading
    LoadingSystem,

    // -----------------
    // Initialize
    // -----------------
    // Despawn various objects from world
    InGameDespawnObjects,
    // Spawn basic light, sky, etc...
    InGameSpawnWorldEnvironment,
    // Spawn 3d terrain objects
    InGameSpawnWorldTerrain,
    // Spawn player entities, player system objects, etc...
    InGameSpawnPlayerStaff,
    // Spawn enemy mobs
    InGameSpawnMobs,
    // Spawn projectiles/debris/effects/etc...
    InGameSpawnProjectilesAndEffects,

    // -----------------
    // Input
    // -----------------
    InGameProcessInput,

    // -----------------
    // Gameplay
    // -----------------
    InGamePrepareWeapons,
    InGameUpdateMovements,
    InGameUpdateCameras,

    // -----------------
    // Sound effects
    // -----------------
    InGamePlaySound,

    // -----------------
    // Draw
    // -----------------

    // Editor windows
    InGameEditorWindowsDraw,

    // Debug draw gizmos
    InGameEditorGizmosDraw,
}
