use bevy::prelude::{Condition, IntoSystem, Res};
use bevy_persistent::Persistent;

use crate::features::res::ResEditorFeaturesState;
use crate::prelude::EditorFeature;

pub fn has_editor_feature(feature: EditorFeature) -> impl Condition<()> {
    IntoSystem::into_system(move |data: Res<Persistent<ResEditorFeaturesState>>| {
        data.has_feature(feature)
    })
}
