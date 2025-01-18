use bevy::prelude::Component;
use brg_fundamental::prelude::CmpTransform2D;

#[derive(Component)]
#[require(CmpTransform2D)]
pub struct CmpLandscapeLoadActorInitiator;
