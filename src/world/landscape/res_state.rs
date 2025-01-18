use bevy::prelude::{Entity, Handle, Mesh, Resource, StandardMaterial};
use bevy::utils::hashbrown::HashMap;
use brg_core::prelude::Chunk;

use super::dto::{LoadedChunk, MeshIdent};

#[derive(Resource, Default)]
pub(crate) struct ResLandscapeState {
    pub(super) terrain:          Option<Entity>,
    pub(super) loaded_chunks:    HashMap<Chunk, LoadedChunk>,
    pub(super) meshes:           HashMap<MeshIdent, Handle<Mesh>>,
    pub(super) terrain_material: Option<Handle<StandardMaterial>>,
}
