use bevy::prelude::Name;
use bevy::prelude::Sprite;
use bevy::prelude::Transform;
use bevy::prelude::{Component, Reflect};

#[derive(Component, Reflect, Default)]
#[require(Transform, Sprite, Name)]
pub struct CmpUnit {}