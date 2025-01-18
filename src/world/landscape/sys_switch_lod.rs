use bevy::prelude::{EventReader, Query, Visibility, With};
use brg_core::prelude::{BlockPosition, Chunk, V2};
use brg_fundamental::prelude::CmpTransform2D;

use super::cmp::CmpLandscapeChild;
use super::cmp_actor_initiator::CmpLandscapeLoadActorInitiator;
use super::enum_lod_level::EChunkLodLevel;
use super::evt_actor_move_in_chunk::EvtActorMoveInChunk;

pub fn sys_switch_lod(
    mut reader: EventReader<EvtActorMoveInChunk>,
    mut chunks_meshes: Query<(&CmpLandscapeChild, &mut Visibility)>,
    initiators: Query<&CmpTransform2D, With<CmpLandscapeLoadActorInitiator>>,
) {
    // if we have at least one event, this mean
    // that any active actor move to some new chunk
    // we don't care about details, and can update state for all of them
    let ev = reader.read().last();
    if ev.is_none() {
        return;
    }

    // extract all initiators positions
    let positions: Vec<V2> = initiators.iter().map(|c| c.position).collect();

    // update visibility for each mesh
    for (mesh, mut visibility) in &mut chunks_meshes {
        let mut min_dist = 4096.0;

        // calculate min dist to every actor
        {
            for pos in &positions {
                let dist = pos.distance(mesh.chunk.position_center());
                if dist < min_dist {
                    min_dist = dist;
                }
            }
        }

        let lod_level = if min_dist <= (Chunk::size() * 8) as f32 {
            EChunkLodLevel::LOD0
        } else {
            EChunkLodLevel::LOD1
        };

        *visibility = match lod_level == mesh.lod {
            true => Visibility::Visible,
            false => Visibility::Hidden,
        };
    }
}
