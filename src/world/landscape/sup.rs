use bevy::ecs::system::SystemParam;
use bevy::prelude::{Assets, Commands, Mesh, ResMut};
use brg_fundamental::prelude::SupHeightmap;
use brg_scene::prelude::SupAssets;

use super::material::TerrainMaterial;
use super::res_state::ResLandscapeState;

#[derive(SystemParam)]
pub struct SupLandscape<'w, 's> {
    pub(super) cmd: Commands<'w, 's>,

    pub(super) heightmap:     SupHeightmap<'w>,
    pub(super) assets:        SupAssets<'w>,
    pub(super) state:         ResMut<'w, ResLandscapeState>,
    pub(super) assets_meshes: ResMut<'w, Assets<Mesh>>,
    pub(super) materials:     ResMut<'w, Assets<TerrainMaterial>>,
}
