use bevy::ecs::system::SystemParam;
use bevy::math::bounding::{Bounded2d, BoundingCircle, BoundingCircleCast, RayCast2d};
use bevy::prelude::{Circle, Isometry2d, Ray2d, Rot2, Vec2};
use brg_core::prelude::V2;

use crate::prelude::{
    CmpCollisionVolume,
    DtoCollisionHit,
    DtoCollisionMovingObject,
    DtoCollisionStaticObject,
};

#[derive(SystemParam)]
pub struct SupRayCastCollision {}

impl SupRayCastCollision {
    pub fn try_move_with_ray_cast(
        &self,
        src: DtoCollisionMovingObject,
        dst: DtoCollisionStaticObject,
    ) -> Option<DtoCollisionHit> {
        // note: only circle to circle is supported right now

        let src_radius = match src.volume {
            CmpCollisionVolume::Circle(m) => m,
        };
        let dst_radius = match dst.volume {
            CmpCollisionVolume::Circle(m) => m,
        };
        let max_dist_src = src.pos_current.distance(src.pos_desired) + src_radius;
        let min_dist_dst = src.pos_current.distance(dst.pos) - dst_radius;

        // fast check that this shapes cannot collide by distance
        if min_dist_dst > max_dist_src {
            return None;
        }

        let dist = src.pos_current.distance(src.pos_desired);
        let direction = src.pos_current.angle_to(src.pos_desired);

        let ray = self.ray_pointed_to(src.pos_current, src.pos_desired);
        let caster_point = RayCast2d::from_ray(ray, dist);
        let caster = BoundingCircleCast {
            circle: BoundingCircle::new(Vec2::ZERO, src_radius),
            ray:    caster_point,
        };

        let dst_iso = Isometry2d::new(dst.pos.as_2d(), Rot2::radians(direction));
        let hit_distance =
            caster.circle_collision_at(Circle::new(dst_radius).bounding_circle(dst_iso));

        match hit_distance {
            None => None,
            Some(distance) => Some(DtoCollisionHit {
                pos: src.pos_current.polar_offset(distance, direction),
            }),
        }
    }

    #[inline(always)]
    fn ray_pointed_to(&self, src: V2, dst: V2) -> Ray2d {
        Ray2d {
            origin:    src.as_2d(),
            direction: src.as_dir2_to(dst),
        }
    }
}
