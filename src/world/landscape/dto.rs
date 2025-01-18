use bevy::prelude::Entity;
use brg_core::prelude::Chunk;

use super::enum_lod_level::EChunkLodLevel;

pub struct LoadedChunk {
    pub lod0: Entity,
    pub lod1: Entity,
}

#[derive(Hash, PartialEq, Eq)]
pub struct MeshIdent {
    pub chunk: Chunk,
    pub lod:   EChunkLodLevel,
}
