use bevy::prelude::{Condition, IntoSystem, Res};

use crate::features::res::ResEditorFeaturesState;
use crate::prelude::EditorFeature;

pub fn has_editor_feature(feature: EditorFeature) -> impl Condition<()> {
    IntoSystem::into_system(move |data: Res<ResEditorFeaturesState>| {
        *data.features.get(&feature.to_string()).unwrap_or(&false)
    })
}
