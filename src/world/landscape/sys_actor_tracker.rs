use bevy::prelude::{Changed, Entity, EventWriter, OnAdd, OnRemove, Query, ResMut, Trigger, With};
use brg_core::prelude::VecExt;
use brg_fundamental::prelude::CmpTransform2D;

use super::cmp_actor_initiator::CmpLandscapeLoadActorInitiator;
use super::evt_actor_move_in_chunk::EvtActorMoveInChunk;
use super::res_actor_tracker::ResActorTracker;

pub fn sys_on_add_tracker_component(
    trigger: Trigger<OnAdd, CmpLandscapeLoadActorInitiator>,
    q: Query<(Entity, &CmpTransform2D), With<CmpLandscapeLoadActorInitiator>>,
    mut tracker: ResMut<ResActorTracker>,
) {
    let (ent, trm) = q.get(trigger.entity()).unwrap();
    let chunk = trm.position.chunk();

    tracker.actor_current_chunks.insert(ent, chunk);
}

pub fn sys_on_remove_tracker_component(
    trigger: Trigger<OnRemove, CmpLandscapeLoadActorInitiator>,
    q: Query<Entity, With<CmpLandscapeLoadActorInitiator>>,
    mut tracker: ResMut<ResActorTracker>,
) {
    let ent = q.get(trigger.entity()).unwrap();
    tracker.actor_current_chunks.remove(&ent);
}

pub fn sys_track_actors(
    q: Query<
        (Entity, &CmpTransform2D),
        (
            With<CmpLandscapeLoadActorInitiator>,
            Changed<CmpTransform2D>,
        ),
    >,
    mut tracker: ResMut<ResActorTracker>,
    mut writer: EventWriter<EvtActorMoveInChunk>,
) {
    for (ent, trm) in &q {
        let new_chunk = trm.position.chunk();
        let Some(prev_chunk) = tracker.actor_current_chunks.get(&ent) else {
            continue;
        };

        let prev_chunk = *prev_chunk;
        if prev_chunk == new_chunk {
            continue;
        };

        tracker.actor_current_chunks.insert(ent, new_chunk);
        writer.send(EvtActorMoveInChunk {
            actor:      ent,
            chunk_prev: prev_chunk,
            chunk_next: new_chunk,
        });
    }
}
