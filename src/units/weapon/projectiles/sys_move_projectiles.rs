use bevy::prelude::{
    Commands,
    DespawnRecursiveExt,
    Entity,
    EventWriter,
    Query,
    Res,
    Time,
    Without,
};
use brg_core::prelude::V2;
use brg_fundamental::prelude::{
    CmpCollisionVolume,
    CmpTransform2D,
    DtoCollisionMovingObject,
    DtoCollisionStaticObject,
    SupRayCastCollision,
};

use super::cmp_projectile::CmpProjectile;
use super::evt::EvtProjectileCollided;
use crate::units::cmp_team::CmpTeam;

pub fn move_projectiles(
    mut cmd: Commands,
    mut query_projectiles: Query<(
        Entity,
        &mut CmpTransform2D,
        &mut CmpProjectile,
        &CmpCollisionVolume,
        &CmpTeam,
    )>,
    query_targets: Query<
        (Entity, &CmpTransform2D, &CmpCollisionVolume, &CmpTeam),
        Without<CmpProjectile>,
    >,
    time: Res<Time>,
    ray_cast_collision: SupRayCastCollision,
    mut writer: EventWriter<EvtProjectileCollided>,
) {
    for (proj_ent, mut proj_trm, mut proj_data, proj_volume, proj_team) in &mut query_projectiles {
        let acceleration = proj_data.acceleration * time.delta_secs();
        proj_data.speed += acceleration;

        let offset = V2::ZERO.polar_offset(
            proj_data.speed.meters_per_second() * time.delta_secs(),
            proj_trm.angle,
        );

        let pos_desired = proj_trm.position + offset;
        let mut collided = false;

        for (target_ent, target_trm, target_volume, target_team) in &query_targets {
            if proj_team.team.is_friendly_with(target_team.team) && !proj_data.friendly_fire {
                continue;
            }

            let hit = ray_cast_collision.try_move_with_ray_cast(
                DtoCollisionMovingObject {
                    pos_current: proj_trm.position,
                    pos_desired,
                    volume: *proj_volume,
                },
                DtoCollisionStaticObject {
                    pos:    target_trm.position,
                    volume: *target_volume,
                },
            );

            match hit {
                None => continue,
                Some(collision) => {
                    collided = true;
                    proj_trm.position = collision.pos;

                    writer.send(EvtProjectileCollided {
                        caster_entity: proj_data.caster,
                        projectile_entity: proj_ent,
                        target_entity: target_ent,
                        caster_team: proj_team.team,
                        target_team: target_team.team,
                        collision,
                        cast: proj_data.hit_spell_cast.clone(),
                        sound: proj_data.hit_sound.clone(),
                    });
                    break;
                }
            }
        }

        match collided {
            false => proj_trm.position = pos_desired,
            true => cmd.entity(proj_ent).despawn_recursive(),
        }
    }
}
