use bevy::color::palettes::tailwind::{GRAY_300, LIME_600, RED_600};
use bevy::prelude::{Query, With};
use brg_fundamental::prelude::{CmpTransform2D, GizmosX};

use super::cmp_projectile::CmpProjectile;
use crate::units::cmp_team::{CmpTeam, ETeam};

pub fn debug_draw_projectiles(
    mut gz: GizmosX,
    q: Query<(&CmpTransform2D, &CmpTeam), With<CmpProjectile>>,
) {
    for (trm, team) in &q {
        gz.point_custom_height(
            trm.position.with_height(trm.height),
            match team.team {
                ETeam::Player => LIME_600,
                ETeam::Enemies => RED_600,
                _ => GRAY_300,
            },
        )
    }
}
