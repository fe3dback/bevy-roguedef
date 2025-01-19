use bevy::prelude::{Condition, IntoSystem, Res};

use super::enums::{SceneFeature, SceneType};

pub fn has_feature(feature: SceneFeature) -> impl Condition<()> {
    IntoSystem::into_system(move |scene: Res<SceneType>| match scene.as_ref() {
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
    })
}
