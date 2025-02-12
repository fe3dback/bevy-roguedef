use bevy::color::palettes::tailwind::{GRAY_300, LIME_600, RED_600};
use bevy::prelude::{Query, With};
use brg_fundamental::prelude::{CmpTransform2D, Point, SupGizmos};

use super::cmp_projectile::CmpProjectile;
use crate::units::cmp_team::{CmpTeam, ETeam};

pub fn debug_draw_projectiles(
    mut gz: SupGizmos,
    q: Query<(&CmpTransform2D, &CmpTeam), With<CmpProjectile>>,
) {
    for (trm, team) in &q {
        gz.point(
            Point::Rel(trm.position.with_height(trm.height)),
            match team.team {
                ETeam::Player => LIME_600,
                ETeam::Enemies => RED_600,
                _ => GRAY_300,
            },
        );
    }
}
