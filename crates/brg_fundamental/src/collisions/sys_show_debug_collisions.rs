use bevy::color::palettes::tailwind::GRAY_700;
use bevy::prelude::Query;

use super::cmp_volume::CmpCollisionVolume;
use crate::prelude::{CmpTransform2D, Point, SupGizmos};

pub fn show_debug_collisions(q: Query<(&CmpTransform2D, &CmpCollisionVolume)>, mut gz: SupGizmos) {
    for (trm, vol) in &q {
        match vol {
            CmpCollisionVolume::Circle(radius) => {
                gz.capsule(
                    Point::Rel(trm.position.with_height(trm.height) + trm.origin_visual_offset),
                    *radius,
                    *radius * 3.0,
                    GRAY_700,
                );
            }
        }
    }
}
