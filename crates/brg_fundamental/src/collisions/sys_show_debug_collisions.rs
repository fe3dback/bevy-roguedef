use bevy::color::palettes::tailwind::GRAY_700;
use bevy::prelude::Query;

use super::cmp_volume::CmpCollisionVolume;
use crate::prelude::{CmpTransform2D, GizmosX};

pub fn show_debug_collisions(q: Query<(&CmpTransform2D, &CmpCollisionVolume)>, mut gz: GizmosX) {
    for (trm, vol) in &q {
        match vol {
            CmpCollisionVolume::Circle(radius) => {
                gz.capsule_custom_height(
                    trm.position.with_height(trm.height) + trm.origin_visual_offset,
                    *radius,
                    *radius * 3.0,
                    GRAY_700,
                );
            }
        }
    }
}
