use bevy::color::palettes::tailwind;
use bevy::color::palettes::tailwind::LIME_50;
use bevy::math::bounding::{Aabb2d, Bounded2d, BoundingCircle, BoundingVolume};
use bevy::prelude::{
    Changed,
    Circle,
    Commands,
    Component,
    Entity,
    Gizmos,
    Isometry2d,
    Or,
    Query,
    Rectangle,
    Reflect,
    Rot2,
    Transform,
};
use bevy_inspector_egui::egui::Shape;

use crate::components::gizmosx::sup::GizmosX;
use crate::components::lib::V2;
use crate::components::transform::CmpTransform2D;

#[derive(Component, Reflect, Debug)]
pub enum CmpCollisionDesiredVolume {
    Aabb(Rectangle),
    Circle(Circle),
}

impl Default for CmpCollisionDesiredVolume {
    fn default() -> Self {
        Self::Circle(Circle::new(1.0))
    }
}

#[derive(Component, Reflect, Debug)]
pub enum CmpCollisionCurrentVolume {
    Aabb(Aabb2d),
    Circle(BoundingCircle),
}

pub fn update_collision_volumes(
    mut commands: Commands,
    query: Query<
        (Entity, &CmpCollisionDesiredVolume, &CmpTransform2D),
        Or<(Changed<CmpCollisionDesiredVolume>, Changed<CmpTransform2D>)>,
    >,
) {
    for (entity, desired_volume, transform) in query.iter() {
        let translation = transform.position.as_2d();
        let isometry = Isometry2d::new(translation, Rot2::radians(transform.angle));
        match desired_volume {
            CmpCollisionDesiredVolume::Aabb(shape) => {
                commands
                    .entity(entity)
                    .try_insert(CmpCollisionCurrentVolume::Aabb(shape.aabb_2d(isometry)));
            }
            CmpCollisionDesiredVolume::Circle(shape) => {
                commands
                    .entity(entity)
                    .try_insert(CmpCollisionCurrentVolume::Circle(
                        shape.bounding_circle(isometry),
                    ));
            }
        }
    }
}

pub fn collision_volumes_draw(mut gizmos: GizmosX, query: Query<(&CmpCollisionCurrentVolume)>) {
    gizmos.rect(V2::new(0.0, 0.0), V2::new(1.0, 1.0), tailwind::BLUE_700);
    gizmos.arrow(V2::ZERO, V2::new(10.0, 0.0), tailwind::RED_700);
    gizmos.arrow(V2::ZERO, V2::new(0.0, 10.0), tailwind::GREEN_700);

    for (volume) in query.iter() {
        match volume {
            CmpCollisionCurrentVolume::Aabb(vol) => {
                let (hw, hh) = (vol.half_size().x, vol.half_size().y);
                let half_size = V2::new(hw, hh);
                let tl = V2::from_2d(vol.center()) - half_size;

                gizmos.rect(tl, half_size * 2.0, tailwind::GRAY_400);
            }
            CmpCollisionCurrentVolume::Circle(vol) => {
                gizmos.circle(V2::from_2d(vol.center()), vol.radius(), tailwind::GRAY_400);
            }
        }
    }
}
