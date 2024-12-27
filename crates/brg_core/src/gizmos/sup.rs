use bevy::ecs::system::SystemParam;
use bevy::math::Isometry3d;
use bevy::prelude::Gizmos;

pub(super) const ISO_IDEN: Isometry3d = Isometry3d::IDENTITY;
pub(super) const DEFAULT_HEIGHT: f32 = 10.0;

#[derive(SystemParam)]
pub struct GizmosX<'w, 's> {
    pub(super) gz: Gizmos<'w, 's>,
}
