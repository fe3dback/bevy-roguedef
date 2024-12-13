use bevy::color::palettes::tailwind;
use bevy::math::Isometry2d;
use bevy::prelude::{Gizmos, UVec2};

use crate::components::lib::V2;
use crate::consts::{PIXELS_PER_METER, PLAYABLE_AREA_SIZE};

pub fn draw_grid(mut gz: Gizmos) {
    // todo: draw if feature enabled
    gz.grid_2d(
        Isometry2d::IDENTITY,
        UVec2::new(100, 100),
        V2::splat(1.0).as_2d(),
        tailwind::ZINC_800,
    )
    .outer_edges();

    gz.circle_2d(
        Isometry2d::IDENTITY,
        PLAYABLE_AREA_SIZE * PIXELS_PER_METER,
        tailwind::SLATE_950,
    );
}
