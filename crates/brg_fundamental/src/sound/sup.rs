use bevy::asset::Assets;
use bevy::ecs::system::SystemParam;
use bevy::prelude::{Commands, Res, ResMut};
use brg_core::prelude::ResRandomSource;
use brg_scene::prelude::AssetSound;

#[derive(SystemParam)]
pub struct SupSound<'w, 's> {
    pub cmd:    Commands<'w, 's>,
    pub sounds: Res<'w, Assets<AssetSound>>,
    pub rnd:    ResMut<'w, ResRandomSource>,
}
