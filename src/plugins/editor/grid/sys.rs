use {
    crate::consts::PLAYABLE_AREA_SIZE,
    bevy::{
        color::palettes::tailwind,
        math::Isometry2d,
        prelude::{Gizmos, UVec2, Vec2},
    },
};

pub fn draw_grid(mut gz: Gizmos) {
    // todo: draw if feature enabled
    gz.grid_2d(
        Isometry2d::IDENTITY,
        UVec2::new(100, 100),
        Vec2::splat(48.0 * 2.0),
        tailwind::SLATE_800,
    )
    .outer_edges();

    gz.circle_2d(
        Isometry2d::IDENTITY,
        PLAYABLE_AREA_SIZE,
        tailwind::SLATE_700,
    );
}
