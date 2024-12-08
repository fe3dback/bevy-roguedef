use bevy::prelude::{Component, Name, Reflect, Sprite, Transform};

#[derive(Component, Reflect, Default)]
#[require(Transform, Sprite, Name)]
pub struct CmpUnit {}
