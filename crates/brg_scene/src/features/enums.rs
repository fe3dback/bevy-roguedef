use bevy::prelude::Resource;

pub const DEFAULT_SCENE: SceneType = SceneType::Editor;

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
    WorldLandscape,
    ExampleCubes,
    Units,
}
