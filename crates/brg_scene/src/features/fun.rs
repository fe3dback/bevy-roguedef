use bevy::prelude::{Condition, IntoSystem, Res};

use super::enums::{SceneFeature, SceneType};

pub fn has_feature(feature: SceneFeature) -> impl Condition<()> {
    IntoSystem::into_system(move |scene: Res<SceneType>| {
        has_feature_in_scene(scene.as_ref(), &feature)
    })
}

pub fn has_feature_in_scene(scene: &SceneType, feature: &SceneFeature) -> bool {
    match scene {
        SceneType::Empty => match feature {
            _ => false,
        },
        SceneType::Game => match feature {
            SceneFeature::Editor => true,
            SceneFeature::EditorGizmos => true,
            SceneFeature::WorldEnvLight => true,
            SceneFeature::ExampleCubes => true,
            SceneFeature::Units => true,
        },
        SceneType::Editor => match feature {
            SceneFeature::Editor => true,
            SceneFeature::EditorGizmos => true,
            SceneFeature::WorldEnvLight => true,
            SceneFeature::ExampleCubes => false,
            SceneFeature::Units => false,
        },
    }
}
