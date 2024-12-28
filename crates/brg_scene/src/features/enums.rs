use bevy::prelude::Resource;

#[derive(Resource, Eq, PartialEq)]
pub enum SceneType {
    Empty,
    Basic,
}

#[derive(Eq, PartialEq, Debug)]
pub enum SceneFeature {
    EditorGizmos,
    WorldEnvLight,
    ExampleCubes,
}
