use bevy::prelude::Component;
use brg_core::prelude::Chunk;

use super::enum_lod_level::EChunkLodLevel;

#[derive(Component)]
pub struct CmpLandscapeRoot;

#[derive(Component)]
#[allow(unused)]
pub struct CmpLandscapeChild {
    pub chunk: Chunk,
    pub lod:   EChunkLodLevel,
}
