use bevy::math::ops;
use bevy::prelude::{Camera, Query, Res, Time, With, Without};
use brg_core::prelude::{lerp, V2};
use brg_fundamental::prelude::{CmpMarkerCameraTarget, CmpTransform2D};

use crate::world::camera::cmp::{CmpCameraAutoFollowSettings, CmpMarkerCameraActive};

pub fn update_game_camera_position(
    mut cam_query: Query<
        (&mut CmpTransform2D, &Camera, &CmpCameraAutoFollowSettings),
        With<CmpMarkerCameraActive>,
    >,
    cam_target_query: Query<
        &CmpTransform2D,
        (With<CmpMarkerCameraTarget>, Without<CmpMarkerCameraActive>),
    >,
    time: Res<Time>,
) {
    let targets_avg_pos = {
        let mut sum_x = 0.0;
        let mut sum_y = 0.0;
        let mut count = 0;

        for target in &cam_target_query {
            sum_x += target.position.x;
            sum_y += target.position.y;
            count += 1;
        }

        if count > 0 {
            V2 {
                x: sum_x / count as f32,
                y: sum_y / count as f32,
            }
        } else {
            V2 { x: 0.0, y: 0.0 }
        }
    };

    for (mut trm, cam, settings) in &mut cam_query {
        if !cam.is_active {
            continue;
        }

        let new_pos = targets_avg_pos + settings.offset;
        let t = 1.0 - ops::exp(-settings.snap_speed * time.delta_secs());

        trm.position = V2::new(
            lerp(trm.position.x, new_pos.x, t),
            lerp(trm.position.y, new_pos.y, t),
        )
    }
}
