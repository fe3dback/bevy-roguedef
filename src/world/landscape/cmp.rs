use bevy::prelude::Component;
use brg_core::prelude::Chunk;

#[derive(Component)]
pub struct CmpLandscapeRoot;

#[derive(Component)]
pub struct CmpLandscapeChild {
    pub chunk: Chunk,
}

#[derive(Component)]
pub struct CmpLandscapeLoadActorInitiator;
