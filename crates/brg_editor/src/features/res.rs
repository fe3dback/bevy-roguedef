use std::collections::HashMap;

use bevy::prelude::Resource;
use bevy::reflect::Reflect;

#[derive(Resource, Default, Reflect)]
pub struct ResEditorFeaturesState {
    pub enabled:  bool,
    pub features: HashMap<String, bool>,
}
