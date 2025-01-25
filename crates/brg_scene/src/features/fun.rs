use bevy::prelude::{App, Condition, IntoSystem, Mut, Res};

use super::enums::{SceneFeature, SceneType};

pub fn has_feature(feature: SceneFeature) -> impl Condition<()> {
    IntoSystem::into_system(move |scene: Res<SceneType>| {
        has_feature_in_scene(scene.as_ref(), &feature)
    })
}

pub fn has_feature_in_app(app: &mut App, feature: SceneFeature) -> bool {
    app.world_mut()
        .resource_scope(|w, scene: Mut<SceneType>| -> bool {
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
            SceneFeature::WorldLandscape => true,
            SceneFeature::ExampleCubes => true,
            SceneFeature::Units => true,
        },
        SceneType::Editor => match feature {
            SceneFeature::Editor => true,
            SceneFeature::EditorGizmos => true,
            SceneFeature::WorldEnvLight => true,
            SceneFeature::WorldLandscape => false,
            SceneFeature::ExampleCubes => false,
            SceneFeature::Units => false,
        },
    }
}
