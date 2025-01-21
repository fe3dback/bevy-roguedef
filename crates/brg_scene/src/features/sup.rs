use bevy::ecs::system::SystemParam;
use bevy::prelude::Res;

use super::enums::SceneType;
use super::fun::has_feature_in_scene;
use crate::prelude::SceneFeature;

#[derive(SystemParam)]
pub struct SupFeatures<'w> {
    pub(super) scene: Res<'w, SceneType>,
}

impl<'w> SupFeatures<'w> {
    #[inline]
    pub fn has_feature(&self, feature: SceneFeature) -> bool {
        has_feature_in_scene(&self.scene, &feature)
    }
}
