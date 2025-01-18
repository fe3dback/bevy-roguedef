use bevy::prelude::{Added, EventReader, Query, With};
use brg_core::prelude::V2;
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
    if entities.is_empty() {
        return;
    }

    let mut just_spawned_actors: Vec<V2> = Vec::with_capacity(4);

    for trm in &entities {
        just_spawned_actors.push(trm.position);
    }

    ls.ensure_chunks_is_loaded_around_actors(just_spawned_actors);
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

    let mut actors: Vec<V2> = Vec::with_capacity(4);

    for trm in &entities {
        actors.push(trm.position);
    }

    ls.ensure_chunks_is_loaded_around_actors(actors);
}
