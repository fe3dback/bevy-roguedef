use bevy::prelude::{Entity, Resource};
use bevy::utils::hashbrown::HashMap;
use brg_core::prelude::Chunk;

#[derive(Resource, Default)]
pub struct ResActorTracker {
    pub(super) actor_current_chunks: HashMap<Entity, Chunk>,
}
