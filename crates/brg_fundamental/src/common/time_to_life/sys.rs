use bevy::prelude::{Commands, DespawnRecursiveExt, Entity, Query, Res, Time};

use super::cmp::CmpTimeToLife;

pub fn remove_expired_ttl_entities(
    mut cmd: Commands,
    mut expired_q: Query<(Entity, &mut CmpTimeToLife)>,
    time: Res<Time>,
) {
    for (ent, mut cmp) in &mut expired_q {
        if cmp.seconds_left > 0.0 {
            cmp.seconds_left -= time.delta().as_secs_f32();
            continue;
        }

        cmd.entity(ent).despawn_recursive();
    }
}
