use bevy::prelude::{Entity, Event};
use brg_core::prelude::Chunk;

#[derive(Event)]
pub struct EvtActorMoveInChunk {
    pub actor:      Entity,
    pub chunk_prev: Chunk,
    pub chunk_next: Chunk,
}
