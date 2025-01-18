use bevy::prelude::{Entity, Handle, Mesh, Resource, StandardMaterial};
use bevy::utils::hashbrown::HashMap;
use brg_core::prelude::Chunk;
use strum::{EnumCount, IntoEnumIterator};

use super::dto::MeshIdent;
use super::enum_lod_level::EChunkLodLevel;

#[derive(Resource)]
pub(crate) struct ResLandscapeState {
    pub(super) terrain:          Option<Entity>,
    pub(super) loaded_chunks:    HashMap<EChunkLodLevel, HashMap<Chunk, Entity>>,
    pub(super) meshes:           HashMap<MeshIdent, Handle<Mesh>>,
    pub(super) terrain_material: Option<Handle<StandardMaterial>>,
}

impl Default for ResLandscapeState {
    fn default() -> Self {
        let mut loaded_chunks: HashMap<EChunkLodLevel, HashMap<Chunk, Entity>> =
            HashMap::with_capacity(EChunkLodLevel::COUNT);
        for lod in EChunkLodLevel::iter() {
            loaded_chunks.insert(lod, HashMap::with_capacity(512));
        }

        Self {
            terrain: None,
            loaded_chunks,
            meshes: HashMap::with_capacity(512),
            terrain_material: None,
        }
    }
}

impl ResLandscapeState {
    pub fn is_chunk_loaded(&self, chunk: Chunk, lod: EChunkLodLevel) -> bool {
        match self.loaded_chunks.get(&lod) {
            Some(loaded_on_lod) => loaded_on_lod.contains_key(&chunk),
            None => false,
        }
    }
}
