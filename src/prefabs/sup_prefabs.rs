use bevy::asset::Assets;
use bevy::ecs::system::SystemParam;
use bevy::pbr::StandardMaterial;
use bevy::prelude::{Commands, Mesh, Res, ResMut};
use brg_scene::prelude::{AssetCreature, AssetProjectile, GameAssets};

#[derive(SystemParam)]
pub struct SupPrefabs<'w, 's> {
    pub(super) cmd: Commands<'w, 's>,

    pub(super) basic_meshes: ResMut<'w, Assets<Mesh>>,
    pub(super) materials:    ResMut<'w, Assets<StandardMaterial>>,

    pub(super) assets:             Res<'w, GameAssets>,
    pub(super) assets_creatures:   Res<'w, Assets<AssetCreature>>,
    pub(super) assets_projectiles: Res<'w, Assets<AssetProjectile>>,
}
