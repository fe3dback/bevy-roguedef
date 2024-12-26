use bevy::color::palettes::css::WHITE_SMOKE;
use bevy::color::palettes::tailwind::{
    self,
    GREEN_300,
    LIME_50,
    LIME_500,
    LIME_800,
    RED_200,
    RED_400,
    RED_700,
};
use bevy::math::bounding::RayCast2d;
use bevy::math::Direction2d;
use bevy::prelude::{
    info,
    ops,
    Commands,
    Component,
    DespawnRecursiveExt,
    Dir2,
    Entity,
    EventWriter,
    Gizmos,
    Query,
    Ray2d,
    Reflect,
    Res,
    Rot2,
    Time,
    Vec2,
    With,
    Without,
};
use brg_core::prelude::V2;

use crate::components::gizmosx::sup::GizmosX;
use crate::components::transform::CmpTransform2D;
use crate::game::collisions::{CmpCollisionCurrentVolume, CmpCollisionDesiredVolume};
use crate::game::damage::{Damage, DamageCastSource, DamageCastTarget, EvtOnDamageCast};
use crate::game::teams::{CmpTeam, Team};

#[derive(Component, Reflect)]
#[require(CmpCollisionDesiredVolume)]
pub struct CmpProjectile {
    pub team:                Team,
    pub caster:              Option<Entity>,
    pub acceleration:        f32,
    pub speed:               f32,
    pub allow_friendly_fire: bool,
    pub damage:              Damage,
}

impl Default for CmpProjectile {
    fn default() -> Self {
        Self {
            team:                Team::default(),
            caster:              None,
            speed:               20.0,
            acceleration:        15.0,
            allow_friendly_fire: true,
            damage:              Damage::default(),
        }
    }
}

pub fn move_projectiles(
    mut cmd: Commands,
    mut damage_writer: EventWriter<EvtOnDamageCast>,
    mut query_projectiles: Query<(Entity, &mut CmpTransform2D, &mut CmpProjectile)>,
    mut gz: Gizmos,
    query_objects: Query<
        (
            Entity,
            &CmpTransform2D,
            &CmpTeam,
            &CmpCollisionCurrentVolume,
        ),
        Without<CmpProjectile>,
    >,
    time: Res<Time>,
) {
    for (bullet_ent, mut bullet_trx, mut bullet) in &mut query_projectiles {
        let mut intersected = false;

        bullet.speed += bullet.acceleration * time.delta().as_secs_f32();
        let distance = bullet.speed * time.delta().as_secs_f32();

        let position_diff = V2::splat(0.0).polar_offset(distance, bullet_trx.angle);
        let pos_cur = bullet_trx.position;
        let pos_next = bullet_trx.position + position_diff;

        for (obj_ent, obj_transform, obj_team, obj_vol) in &query_objects {
            if bullet_trx.position.distance(obj_transform.position) > 5.0 {
                continue;
            }

            if bullet.team.is_friendly_with(obj_team.team) {
                if !bullet.allow_friendly_fire {
                    continue;
                }
            }

            let ray = Ray2d {
                origin:    pos_cur.as_2d(),
                direction: Dir2::new_unchecked(
                    V2::ZERO.polar_offset(1.0, bullet_trx.angle).as_2d(),
                ),
            };
            let ray_cast = RayCast2d::from_ray(ray, distance);
            let intersect = match obj_vol {
                CmpCollisionCurrentVolume::Aabb(vol) => ray_cast.aabb_intersection_at(vol),
                CmpCollisionCurrentVolume::Circle(vol) => ray_cast.circle_intersection_at(vol),
            };

            if intersect.is_none() {
                continue;
            }

            intersected = true;
            let intersect_point =
                V2::from_2d(ray_cast.ray.origin + ray_cast.ray.direction * intersect.unwrap());

            damage_writer.send(EvtOnDamageCast {
                cast:   DamageCastSource {
                    projectile: Some(bullet_ent),
                    caster:     bullet.caster,
                    owner_team: bullet.team,
                    origin:     intersect_point,
                    damage:     bullet.damage,
                },
                target: DamageCastTarget {
                    targets: vec![obj_ent],
                },
            });

            match cmd.get_entity(bullet_ent) {
                Some(e) => e.despawn_recursive(),
                None => {}
            }
        }

        if intersected {
            continue;
        }

        gz.line_2d(
            bullet_trx.position.as_2d(),
            pos_next.as_2d(),
            match bullet.team {
                Team::Player => LIME_500,
                Team::Enemies => RED_400,
                _ => WHITE_SMOKE,
            },
        );
        bullet_trx.position = pos_next;
    }
}

pub fn draw_projectiles(
    mut gizmos: GizmosX,
    query_projectiles: Query<&CmpTransform2D, With<CmpProjectile>>,
) {
    for trx in &query_projectiles {
        gizmos.circle(trx.position, 10.0, tailwind::LIME_800);
    }
}
