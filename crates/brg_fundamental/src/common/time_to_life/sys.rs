use bevy::prelude::{Commands, DespawnRecursiveExt, Entity, Query, Res, Time};

use super::cmp::CmpTimeToLife;

pub fn remove_expired_ttl_entities(
    mut cmd: Commands,
    mut expired_q: Query<(Entity, &mut CmpTimeToLife)>,
    time: Res<Time>,
) {
    for (ent, mut cmp) in &mut expired_q {
        if !cmp.left.is_zero() {
            cmp.left = cmp.left.checked_sub(time.delta()).unwrap_or_default();
            continue;
        }

        cmd.entity(ent).despawn_recursive();
    }
}
