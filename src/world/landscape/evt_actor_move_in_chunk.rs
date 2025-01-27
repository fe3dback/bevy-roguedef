use bevy::prelude::{Entity, Event};
use brg_core::prelude::Chunk;

#[allow(unused)]
#[derive(Event)]
pub struct EvtActorMoveInChunk {
    pub actor:      Entity,
    pub chunk_prev: Chunk,
    pub chunk_next: Chunk,
}
