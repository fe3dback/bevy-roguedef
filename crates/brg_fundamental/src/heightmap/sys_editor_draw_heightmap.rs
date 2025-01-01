use bevy::color::palettes::tailwind::{RED_900, SKY_300};
use bevy::color::Color;
use bevy::prelude::{Mix, Query, Res, With};
use brg_core::prelude::consts::TERRAIN_HEIGHT;
use brg_core::prelude::{Range, VecExt, V2};

use crate::prelude::{CmpMarkerCameraTarget, CmpTransform2D, GizmosX, ResHeightmap};

pub fn editor_draw_heightmap_around_player(
    mut gz: GizmosX,
    hm: Res<ResHeightmap>,
    cam_target_query: Query<&CmpTransform2D, With<CmpMarkerCameraTarget>>,
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

    let target_tile = targets_avg_pos.tile();
    let range = Range::new(
        target_tile.x - 25,
        target_tile.y - 25,
        target_tile.x + 25,
        target_tile.y + 25,
    );
    let color_low = RED_900;
    let color_high = SKY_300;

    for tile in &range {
        let height_percent = hm.height_at_pos(tile.position()) / TERRAIN_HEIGHT;
        let height_color = Color::mix(&color_low.into(), &color_high.into(), height_percent);

        gz.point(tile.position().with_height(0.05), height_color);
    }
}
