use bevy::ecs::system::SystemParam;
use bevy::prelude::Res;
use bevy_persistent::Persistent;

use super::res::ResEditorFeaturesState;
use crate::prelude::EditorFeature;

#[derive(SystemParam)]
pub struct SupEditorFeatures<'w> {
    state: Res<'w, Persistent<ResEditorFeaturesState>>,
}

impl<'w> SupEditorFeatures<'w> {
    pub fn has_feature(&self, feature: EditorFeature) -> bool {
        self.state.has_feature(feature)
    }
}
