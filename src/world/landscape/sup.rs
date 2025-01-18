use bevy::ecs::system::SystemParam;
use bevy::prelude::{Assets, Commands, Mesh, ResMut, StandardMaterial};
use brg_fundamental::prelude::SupHeightmap;

use super::res_state::ResLandscapeState;

#[derive(SystemParam)]
pub struct SupLandscape<'w, 's> {
    pub(super) cmd: Commands<'w, 's>,

    pub(super) hm:            SupHeightmap<'w>,
    pub(super) state:         ResMut<'w, ResLandscapeState>,
    pub(super) assets_meshes: ResMut<'w, Assets<Mesh>>,
    pub(super) materials:     ResMut<'w, Assets<StandardMaterial>>,
}
