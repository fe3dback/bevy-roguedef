#![allow(non_camel_case_types)]

use bevy::prelude::SystemSet;
use strum::{Display, EnumIter};

pub const MAGIC_NOT_ON_PAUSE: &str = "NOT_ON_PAUSE";
pub const MAGIC_EDITOR_ONLY: &str = "EDITOR_ONLY";
pub const MAGIC_ALLOW_ON_LOADING: &str = "ALLOW_ON_LOAD";

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone, Copy, EnumIter, Display)]
pub enum GameSystemSet {
    // note: Order of enum values is matter
    //       systems will run in this order:

    // note: Prefix is important!
    //       use prefix "InGame" - for bind check `in_state(InGame)` to this set

    // For loading game assets
    ALLOW_ON_LOAD__LoadingAssets,

    // For spawning SYSTEM/staff/special entities
    ALLOW_ON_LOAD__LoadingSystem,

    // -----------------
    // Editor
    // -----------------

    // editor-only, when changing game state (like pause/resume, save/load, etc..)
    EDITOR_ONLY__ChangeGlobalGameState,

    // draw editor only egui panels
    EDITOR_ONLY__DrawEditorEguiPanels,

    // -----------------
    // Initialize
    // -----------------
    // Despawn various objects from world
    NOT_ON_PAUSE__DespawnObjects,
    // Spawn basic light, sky, etc...
    SpawnWorldEnvironment,
    // Spawn 3d terrain objects
    SpawnWorldTerrain,
    // Spawn player entities, player system objects, etc...
    NOT_ON_PAUSE__SpawnPlayerStaff,
    // Spawn enemy mobs
    NOT_ON_PAUSE__SpawnMobs,
    // Spawn projectiles/debris/effects/etc...
    NOT_ON_PAUSE__SpawnProjectilesAndEffects,

    // -----------------
    // Input
    // -----------------
    NOT_ON_PAUSE__ProcessInput,

    // -----------------
    // Gameplay
    // -----------------
    NOT_ON_PAUSE__PrepareWeapons,
    NOT_ON_PAUSE__UpdateMovements,
    NOT_ON_PAUSE__CastSpells,
    NOT_ON_PAUSE__UpdateGameCameras,
    UpdateDebugCameras,

    // -----------------
    // Gameplay finalization
    // -----------------
    NOT_ON_PAUSE__UpdateGameplayCaches,

    // -----------------
    // Sound effects
    // -----------------
    NOT_ON_PAUSE__PlaySound,

    // -----------------
    // Draw
    // -----------------

    // Debug draw gizmos
    GizmosDraw,
}
