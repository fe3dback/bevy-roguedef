use bevy::prelude::{
    Camera,
    Commands,
    Component,
    DespawnRecursiveExt,
    Entity,
    GlobalTransform,
    Query,
    Reflect,
    ReflectResource,
    Res,
    ResMut,
    Resource,
    Time,
    Window,
    With,
};
use bevy::window::PrimaryWindow;
use brg_core::prelude::V2;
use rand_chacha::ChaCha8Rng;

#[derive(Resource)]
pub struct ResRandomSource {
    pub rnd: ChaCha8Rng,
}
