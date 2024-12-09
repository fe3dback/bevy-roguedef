use {
    crate::{
        components::{lib::V2, transform::CmpTransform2D},
        game::{
            collisions::{CmpCollisionCurrentVolume, CmpCollisionDesiredVolume},
            damage::{Damage, DamageCastSource},
            teams::Team,
        },
    },
    bevy::{
        color::palettes::tailwind::{self, LIME_50, LIME_800},
        math::bounding::RayCast2d,
        prelude::{
            info,
            ops,
            Commands,
            Component,
            Dir2,
            Gizmos,
            Query,
            Ray2d,
            Reflect,
            Res,
            Time,
            Vec2,
            With,
            Without,
        },
    },
};

#[derive(Component, Reflect)]
#[require(CmpCollisionDesiredVolume)]
pub struct CmpProjectile {
    pub team:                Team,
    pub acceleration:        f32,
    pub speed:               f32,
    pub allow_friendly_fire: bool,
    pub damage:              Damage,
}

impl Default for CmpProjectile {
    fn default() -> Self {
        Self {
            team:                Team::default(),
            speed:               1000.0,
            acceleration:        500.0,
            allow_friendly_fire: false,
            damage:              Damage::default(),
        }
    }
}

pub fn move_projectiles(
    mut gizmos: Gizmos,
    mut query_projectiles: Query<(&mut CmpTransform2D, &mut CmpProjectile)>,
    query_objects: Query<(&CmpTransform2D, &CmpCollisionCurrentVolume), Without<CmpProjectile>>,
    time: Res<Time>,
) {
    for (mut bullet_trx, mut bullet) in &mut query_projectiles {
        let mut intersected = false;

        bullet.speed += bullet.acceleration * time.delta().as_secs_f32();
        let distance = bullet.speed * time.delta().as_secs_f32();

        let position_diff = V2::splat(0.0).polar_offset(distance, bullet_trx.angle);
        let pos_cur = bullet_trx.position;
        let pos_next = bullet_trx.position + position_diff;

        for (obj_transform, obj_vol) in &query_objects {
            if bullet_trx.position.distance(obj_transform.position) > 256.0 {
                continue;
            }

            // todo: check friendly fire

            let ray = Ray2d {
                origin:    pos_cur.as_2d(),
                direction: Dir2::new_unchecked(
                    V2::ZERO.polar_offset(1.0, bullet_trx.angle).as_2d(),
                ),
            };
            let ray_cast = RayCast2d::from_ray(ray, distance);
            draw_ray(&mut gizmos, &ray_cast); // todo: remove

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

            let damage_cast = DamageCastSource {
                caster:     None, // todo: entity,
                owner_team: bullet.team,
                origin:     intersect_point,
                damage:     bullet.damage,
            };

            // todo: send event

            // todo: delete this bullet entity
        }

        if intersected {
            continue;
        }

        bullet_trx.position = pos_next;
    }
}

// todo: remove this
fn draw_ray(gizmos: &mut Gizmos, ray: &RayCast2d) {
    gizmos.line_2d(
        ray.ray.origin,
        ray.ray.origin + *ray.ray.direction * ray.max,
        tailwind::RED_700,
    );
}

pub fn draw_projectiles(
    mut gizmos: Gizmos,
    query_projectiles: Query<&CmpTransform2D, With<CmpProjectile>>,
) {
    for (trx) in &query_projectiles {
        gizmos.circle_2d(trx.position.as_2d(), 10.0, tailwind::LIME_800);
    }
}
