use bevy::prelude::{EventWriter, Res, ResMut};
use brg_core::prelude::{Chunk, VecExt};
use brg_fundamental::prelude::{ResCoords, SupHeightmap};

use super::evt_actor_move_in_chunk::EvtLodPoeMovedIntoNewChunk;
use super::res_state::ResLandscapeState;

pub fn sys_update_lod_poe(
    coord: Res<ResCoords>,
    mut hm: SupHeightmap,
    mut state: ResMut<ResLandscapeState>,
    mut writer: EventWriter<EvtLodPoeMovedIntoNewChunk>,
) {
    let origin = coord.camera_origin.xy();
    let target = coord.camera_target.xy();
    let dist = origin.distance(target);
    let max_dist = (coord.camera_origin.h - hm.height_at_pos(origin)) * 2.0;

    let prev_chunk = state.lod_point_of_interest.chunk();
    let origin_dist_diff = coord
        .camera_origin
        .as_3d()
        .distance(state.lod_last_origin.as_3d());

    // update state
    state.lod_point_of_interest = match dist < max_dist {
        true => target,
        false => origin.polar_offset(max_dist, origin.angle_to(target)),
    };

    // calculate real 3d distance from camera origin to computed POE
    let dist_to_poe = coord.camera_origin.as_3d().distance(
        state
            .lod_point_of_interest
            .with_height(hm.height_at_pos(state.lod_point_of_interest))
            .as_3d(),
    );

    let next_chunk = state.lod_point_of_interest.chunk();
    if prev_chunk != next_chunk || origin_dist_diff > Chunk::size_m().x / 2.0 {
        state.lod_last_origin = coord.camera_origin;
        writer.send(EvtLodPoeMovedIntoNewChunk {
            chunk_prev: prev_chunk,
            chunk_next: next_chunk,
            dist_to_poe,
        });
    }
}
