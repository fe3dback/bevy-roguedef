use bevy::color::palettes::tailwind::{BLUE_900, GREEN_900, ORANGE_800, RED_900, SKY_300};
use bevy::color::Color;
use bevy::prelude::{Mix, Query, Res, With};
use brg_core::prelude::{BlockPosition, Range, Tile, VecExt, V2};

use super::res::ResLandscape;
use crate::prelude::{CmpMarkerCameraTarget, CmpTransform2D, GizmosX};

pub fn editor_draw_heightmap_around_player(
    mut gz: GizmosX,
    cam_target_query: Query<&CmpTransform2D, With<CmpMarkerCameraTarget>>,
    land: Res<ResLandscape>,
) {
    let half_w = land.width as f32 / 2.0 + 1.0;
    let half_h = land.height as f32 / 2.0 + 1.0;
    let z = land.volume as f32;

    let tl = V2::new(-half_w, -half_h);
    let tr = V2::new(half_w, -half_h);
    let bl = V2::new(-half_w, half_h);
    let br = V2::new(half_w, half_h);

    // draw bounding box
    {
        // lower
        gz.line(tl, tr, RED_900);
        gz.line(tr, br, ORANGE_800);
        gz.line(br, bl, BLUE_900);
        gz.line(bl, tl, GREEN_900);

        // upper
        gz.line_custom_height(tl.with_height(z), tr.with_height(z), RED_900);
        gz.line_custom_height(tr.with_height(z), br.with_height(z), ORANGE_800);
        gz.line_custom_height(br.with_height(z), bl.with_height(z), BLUE_900);
        gz.line_custom_height(bl.with_height(z), tl.with_height(z), GREEN_900);

        // bottom to up lines
        gz.line_custom_height(tl.with_height(0.0), tl.with_height(z), RED_900);
        gz.line_custom_height(tr.with_height(0.0), tr.with_height(z), ORANGE_800);
        gz.line_custom_height(br.with_height(0.0), br.with_height(z), BLUE_900);
        gz.line_custom_height(bl.with_height(0.0), bl.with_height(z), GREEN_900);
    }

    // draw height points

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
    let range = Range::<Tile>::new(
        target_tile.x - 25,
        target_tile.y - 25,
        target_tile.x + 25,
        target_tile.y + 25,
    );
    let color_low = RED_900;
    let color_high = SKY_300;

    for tile in &range {
        let height_percent = gz.heightmap.height_at_pos(tile.position_tl()) / land.volume as f32;
        let height_color = Color::mix(&color_low.into(), &color_high.into(), height_percent);

        gz.point_custom_height(tile.position_tl().with_height(0.05), height_color);
    }
}
