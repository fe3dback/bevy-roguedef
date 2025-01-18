use brg_core::prelude::Chunk;

use super::enum_lod_level::EChunkLodLevel;

#[derive(Hash, PartialEq, Eq)]
pub struct MeshIdent {
    pub chunk: Chunk,
    pub lod:   EChunkLodLevel,
}
