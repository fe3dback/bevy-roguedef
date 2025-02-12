use bevy::color::palettes::tailwind::{RED_900, SKY_300, SKY_950};
use bevy::color::Color;
use bevy::prelude::{Mix, Query, Res, With};
use brg_core::prelude::{BlockPosition, Range, Tile, VecExt, V2};

use super::res::ResLandscape;
use crate::prelude::{CmpMarkerCameraTarget, CmpTransform2D, Point, SupGizmos};

pub fn editor_draw_heightmap_around_player(
    mut gz: SupGizmos,
    cam_target_query: Query<&CmpTransform2D, With<CmpMarkerCameraTarget>>,
    land: Res<ResLandscape>,
) {
    let half_w = land.width as f32 / 2.0;
    let half_h = land.height as f32 / 2.0;
    let z = land.volume as f32;

    let tl = V2::new(-half_w, -half_h);
    let tr = V2::new(half_w, -half_h);
    let bl = V2::new(-half_w, half_h);
    let br = V2::new(half_w, half_h);

    // draw bounding box
    {
        // lower
        gz.line(Point::Abs(tl), Point::Abs(tr), SKY_950);
        gz.line(Point::Abs(tr), Point::Abs(br), SKY_950);
        gz.line(Point::Abs(br), Point::Abs(bl), SKY_950);
        gz.line(Point::Abs(bl), Point::Abs(tl), SKY_950);

        // upper
        gz.line(
            Point::Abs(tl.with_height(z)),
            Point::Abs(tr.with_height(z)),
            SKY_950,
        );
        gz.line(
            Point::Abs(tr.with_height(z)),
            Point::Abs(br.with_height(z)),
            SKY_950,
        );
        gz.line(
            Point::Abs(br.with_height(z)),
            Point::Abs(bl.with_height(z)),
            SKY_950,
        );
        gz.line(
            Point::Abs(bl.with_height(z)),
            Point::Abs(tl.with_height(z)),
            SKY_950,
        );

        // bottom to up lines
        gz.line(
            Point::Abs(tl.with_height(0.0)),
            Point::Abs(tl.with_height(z)),
            SKY_300,
        );
        gz.line(
            Point::Abs(tr.with_height(0.0)),
            Point::Abs(tr.with_height(z)),
            SKY_950,
        );
        gz.line(
            Point::Abs(br.with_height(0.0)),
            Point::Abs(br.with_height(z)),
            SKY_950,
        );
        gz.line(
            Point::Abs(bl.with_height(0.0)),
            Point::Abs(bl.with_height(z)),
            SKY_950,
        );
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

        gz.point(
            Point::Rel(tile.position_tl().with_height(0.05)),
            height_color,
        );
    }
}
