use bevy::prelude::Component;
use bevy::prelude::Sprite;
use bevy::prelude::Transform;

#[derive(Component)]
#[require(Transform, Sprite)]
pub struct CmpMarkerPlayer {}
