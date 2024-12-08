use bevy::prelude::Component;
use bevy::prelude::Name;
use bevy::prelude::Sprite;
use bevy::prelude::Transform;

#[derive(Component)]
#[require(Transform, Sprite, Name)]
pub struct CmpMarkerPlayer {}
