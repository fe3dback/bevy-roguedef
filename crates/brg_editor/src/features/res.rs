use std::collections::HashMap;

use bevy::prelude::Resource;
use bevy::reflect::Reflect;
use serde::{Deserialize, Serialize};

use crate::prelude::EditorFeature;

#[derive(Resource, Default, Reflect, Serialize, Deserialize)]
pub struct ResEditorFeaturesState {
    pub enabled:  bool,
    pub features: HashMap<String, bool>,
}

impl ResEditorFeaturesState {
    #[inline]
    pub fn has_feature(&self, feature: EditorFeature) -> bool {
        *self.features.get(&feature.to_string()).unwrap_or(&false)
    }
}
