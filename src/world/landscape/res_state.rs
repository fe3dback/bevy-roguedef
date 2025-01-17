use bevy::prelude::{Entity, Handle, Mesh, Resource};
use bevy::utils::hashbrown::HashMap;
use brg_core::prelude::Chunk;

#[derive(Resource, Default)]
pub(crate) struct ResLandscapeState {
    pub terrain:       Option<Entity>,
    pub loaded_chunks: HashMap<Chunk, Entity>,
    pub meshes:        HashMap<Chunk, Handle<Mesh>>,
}
