use bevy::prelude::{Changed, Commands, DespawnRecursiveExt, Entity, Query};

use super::cmp_health::CmpHealth;

pub fn despawn_on_death(mut cmd: Commands, q: Query<(Entity, &CmpHealth), Changed<CmpHealth>>) {
    for (ent, health) in &q {
        if health.alive {
            continue;
        }

        cmd.entity(ent).despawn_recursive();
    }
}
