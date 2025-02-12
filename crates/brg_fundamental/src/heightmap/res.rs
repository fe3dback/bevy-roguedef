use bevy::prelude::{ReflectResource, Resource};
use bevy::reflect::Reflect;
use brg_core::prelude::{Tile, V2};

#[derive(Resource, Default, Reflect)]
#[reflect(Resource)]
pub struct ResLandscape {
    pub width:  u32, // x axis
    pub height: u32, // y axis
    pub volume: u32, // z axis
    pub offset: Tile,
    #[reflect(ignore)]
    pub values: Vec<f32>,
}
