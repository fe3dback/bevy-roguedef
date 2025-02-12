use bevy::prelude::Event;
use brg_core::prelude::Chunk;

#[allow(unused)]
#[derive(Event)]
pub struct EvtLodPoeMovedIntoNewChunk {
    pub chunk_prev:  Chunk,
    pub chunk_next:  Chunk,
    pub dist_to_poe: f32,
}
