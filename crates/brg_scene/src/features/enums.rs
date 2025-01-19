use bevy::prelude::Resource;

#[derive(Resource, Eq, PartialEq)]
pub enum SceneType {
    Empty,
    Game,
    Editor,
}

#[derive(Eq, PartialEq, Debug)]
pub enum SceneFeature {
    Editor,
    EditorGizmos,
    WorldEnvLight,
    ExampleCubes,
    Units,
}
