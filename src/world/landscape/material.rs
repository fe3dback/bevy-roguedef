use bevy::prelude::{Asset, Handle, Image, Material, TypePath, Vec2};
use bevy::render::render_resource::{AsBindGroup, ShaderRef};
use brg_core::prelude::V2;

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct TerrainMaterial {
    #[uniform(0)]
    pub map_size: Vec2,

    #[texture(1)]
    #[sampler(2)]
    pub world_albedo: Handle<Image>,

    #[texture(3)]
    #[sampler(4)]
    pub texture_grass: Handle<Image>,
    // #[texture(0, dimension = "2d_array")]
    // #[sampler(1)]
    // pub textures: Handle<Image>,
    // #[uniform(2)]
    // pub color: Color,
    // #[uniform(3)]
    // pub max_height: f32,
}

impl TerrainMaterial {
    pub fn new(map_size: V2, world_albedo: Handle<Image>, texture_grass: Handle<Image>) -> Self {
        Self {
            map_size: map_size.as_2d_ui(),
            world_albedo,
            texture_grass,
            // color: Color::WHITE,
            // max_height: TERRAIN_MAX_HEIGHT,
            // textures: textures_2d_arr,
        }
    }
}

impl Material for TerrainMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/terrain.wgsl".into()
    }
}
