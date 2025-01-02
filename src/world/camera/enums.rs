use bevy::prelude::{Component, Reflect};

#[derive(Component, Debug, Reflect, Default, Ord, PartialOrd, Eq, PartialEq, Clone, Copy)]
pub enum CmpCameraType {
    EditorFly,
    #[default]
    GameTopDown,
}
