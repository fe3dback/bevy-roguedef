use {
    crate::components::transform::CmpTransform2D,
    bevy::{
        color::palettes::{tailwind, tailwind::LIME_50},
        math::bounding::{Aabb2d, Bounded2d, BoundingCircle, BoundingVolume},
        prelude::{
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
        },
    },
    bevy_inspector_egui::egui::Shape,
};

#[derive(Component, Reflect, Debug)]
pub enum CmpCollisionDesiredVolume {
    Aabb(Rectangle),
    Circle(Circle),
}

impl Default for CmpCollisionDesiredVolume {
    fn default() -> Self {
        Self::Circle(Circle::new(16.0))
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

pub fn collision_volumes_draw(mut gizmos: Gizmos, query: Query<(&CmpCollisionCurrentVolume)>) {
    for (volume) in query.iter() {
        match volume {
            CmpCollisionCurrentVolume::Aabb(vol) => {
                gizmos.rect_2d(vol.center(), vol.half_size() * 2., tailwind::GRAY_400);
            }
            CmpCollisionCurrentVolume::Circle(vol) => {
                gizmos.circle_2d(vol.center(), vol.radius(), tailwind::GRAY_400);
            }
        }
    }
}
