use bevy::ecs::system::SystemParam;
use bevy::math::Isometry3d;
use bevy::prelude::{Gizmos, Res};

use crate::prelude::ResHeightmap;

pub const ISO_IDEN: Isometry3d = Isometry3d::IDENTITY;

#[derive(SystemParam)]
pub struct GizmosX<'w, 's> {
    pub(super) gz: Gizmos<'w, 's>,

    pub(super) heightmap: Res<'w, ResHeightmap>,
}
