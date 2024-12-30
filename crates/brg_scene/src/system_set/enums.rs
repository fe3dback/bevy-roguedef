use bevy::prelude::SystemSet;
use strum::{Display, EnumIter};

pub const MAGIC_PREFIX_CONDITION_IN_STATE_IN_GAME: &str = "InGame";

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

    // Spawn basic light, sky, etc...
    InGameSpawnWorldEnvironment,
    // Spawn 3d terrain objects
    InGameSpawnWorldTerrain,
    // Spawn player entities, player system objects, etc...
    InGameSpawnPlayerStaff,

    // -----------------
    // Input
    // -----------------
    InGameProcessInput,

    // -----------------
    // Gameplay
    // -----------------
    InGameUpdateMovements,
    InGameUpdateCameras,

    // -----------------
    // Draw
    // -----------------

    // Debug draw gizmos
    InGameEditorGizmosDraw,
}
