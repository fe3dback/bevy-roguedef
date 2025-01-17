use std::collections::VecDeque;

use bevy::prelude::{ReflectResource, Resource};
use bevy::reflect::Reflect;
use bevy::utils::hashbrown::HashSet;
use bevy::utils::HashMap;
use brg_core::prelude::types::NormalizedF32;
use brg_core::prelude::{Chunk, CntOfAreas, Tile, T_LIB_CONT_SIZE_SQ, T_LIB_TILE_SIZE_SQ, V2};

use super::dto_landscape::Landscape;

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct ResHeightmapCache {
    pub(super) tiles:         HashMap<Tile, NormalizedF32>,
    pub(super) queue:         VecDeque<Chunk>,
    pub(super) loaded_chunks: HashSet<Chunk>,
}

#[derive(Resource, Default, Reflect)]
#[reflect(Resource)]
pub struct ResLandscape {
    pub width:     CntOfAreas,
    pub height:    CntOfAreas,
    #[reflect(ignore)]
    pub landscape: Landscape,
}

impl Default for ResHeightmapCache {
    fn default() -> Self {
        Self {
            tiles:         HashMap::default(),
            queue:         VecDeque::with_capacity(T_LIB_CONT_SIZE_SQ),
            loaded_chunks: HashSet::with_capacity(T_LIB_CONT_SIZE_SQ),
        }
    }
}
