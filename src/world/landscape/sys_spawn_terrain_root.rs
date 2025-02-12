use bevy::prelude::EventReader;
use brg_core::prelude::{BlockPosition, V2};

use super::evt_actor_move_in_chunk::EvtLodPoeMovedIntoNewChunk;
use super::sup::SupLandscape;

pub fn sys_spawn_terrain_root(mut ls: SupLandscape) {
    ls.spawn_terrain();
    ls.ensure_chunks_is_loaded_around_poe(V2::ZERO, 0.0);
}

pub fn sys_despawn_terrain_root(mut ls: SupLandscape) {
    ls.despawn_terrain();
}

pub fn sys_respawn_chunks_when_lod_poe_changed(
    mut ls: SupLandscape,
    mut reader: EventReader<EvtLodPoeMovedIntoNewChunk>,
) {
    let ev = reader.read().last();
    if ev.is_none() {
        return;
    }

    let ev = ev.unwrap();
    ls.ensure_chunks_is_loaded_around_poe(ev.chunk_next.position_center(), ev.dist_to_poe);
}
