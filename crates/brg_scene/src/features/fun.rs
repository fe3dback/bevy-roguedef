use bevy::prelude::{Condition, IntoSystem, Res};

use super::enums::{SceneFeature, SceneType};

pub fn has_feature(feature: SceneFeature) -> impl Condition<()> {
    IntoSystem::into_system(move |scene: Res<SceneType>| match scene.as_ref() {
        SceneType::Empty => false,
        SceneType::Basic => match feature {
            SceneFeature::WorldEnvLight => true,
            SceneFeature::ExampleCubes => true,
        },
    })
}
