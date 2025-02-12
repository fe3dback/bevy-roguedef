use bevy::color::palettes::tailwind::{GRAY_500, ROSE_700};
use bevy::prelude::{Query, Transform};
use bevy::render::primitives::Aabb;
use brg_core::prelude::V3;

use crate::prelude::{Point, SupGizmos};

pub fn sys_show_debug_aabb(q: Query<(&Aabb, &Transform)>, mut gz: SupGizmos) {
    let color = GRAY_500;
    let color_tl = ROSE_700;

    for (aabb, trm) in &q {
        let center = V3::from_3d(trm.translation) + V3::from_3d(aabb.center.into());
        let half = V3::from_3d(aabb.half_extents.into());
        let half_height = V3::new(0.0, 0.0, half.h);

        let tl = center - V3::new(half.x, half.y, 0.0);
        let tr = tl + V3::new(half.x * 2.0, 0.0, 0.0);
        let bl = tl + V3::new(0.0, half.y * 2.0, 0.0);
        let br = tl + V3::new(half.x * 2.0, half.y * 2.0, 0.0);

        let top_tl = tl + half_height;
        let bottom_tl = tl - half_height;
        let top_tr = tr + half_height;
        let bottom_tr = tr - half_height;
        let top_bl = bl + half_height;
        let bottom_bl = bl - half_height;
        let top_br = br + half_height;
        let bottom_br = br - half_height;

        // draw lower box
        gz.line(Point::Abs(bottom_tl), Point::Abs(bottom_tr), color);
        gz.line(Point::Abs(bottom_tr), Point::Abs(bottom_br), color);
        gz.line(Point::Abs(bottom_br), Point::Abs(bottom_bl), color);
        gz.line(Point::Abs(bottom_bl), Point::Abs(bottom_tl), color);

        // draw upper box
        gz.line(Point::Abs(top_tl), Point::Abs(top_tr), color);
        gz.line(Point::Abs(top_tr), Point::Abs(top_br), color);
        gz.line(Point::Abs(top_br), Point::Abs(top_bl), color);
        gz.line(Point::Abs(top_bl), Point::Abs(top_tl), color);

        // draw connections
        gz.line(Point::Abs(bottom_tl), Point::Abs(top_tl), color_tl);
        gz.line(Point::Abs(bottom_tr), Point::Abs(top_tr), color);
        gz.line(Point::Abs(bottom_bl), Point::Abs(top_bl), color);
        gz.line(Point::Abs(bottom_br), Point::Abs(top_br), color);
    }
}
