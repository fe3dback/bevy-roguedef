use bevy::prelude::{Added, EventReader, Query, With};
use brg_fundamental::prelude::CmpTransform2D;

use super::cmp_actor_initiator::CmpLandscapeLoadActorInitiator;
use super::evt_actor_move_in_chunk::EvtActorMoveInChunk;
use super::sup::SupLandscape;

pub fn sys_spawn_terrain_root(mut ls: SupLandscape) {
    ls.spawn_terrain();
}

pub fn sys_despawn_terrain_root(mut ls: SupLandscape) {
    ls.despawn_terrain();
}

pub fn sys_spawn_initial_chunks(
    mut ls: SupLandscape,
    entities: Query<&CmpTransform2D, Added<CmpLandscapeLoadActorInitiator>>,
) {
    if let Some(trm) = entities.iter().last() {
        ls.ensure_chunks_is_loaded_around_actors(trm.position);
    }
}

pub fn sys_spawn_chunks_on_actor_moves(
    mut ls: SupLandscape,
    mut reader: EventReader<EvtActorMoveInChunk>,
    entities: Query<&CmpTransform2D, With<CmpLandscapeLoadActorInitiator>>,
) {
    // if we have at least one event, this mean
    // that any active actor move to some new chunk
    // we don't care about details, and can update state for all of them
    let ev = reader.read().last();
    if ev.is_none() {
        return;
    }

    if let Some(trm) = entities.iter().last() {
        ls.ensure_chunks_is_loaded_around_actors(trm.position);
    }
}
