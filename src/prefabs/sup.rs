use bevy::ecs::system::SystemParam;
use bevy::prelude::Commands;

#[derive(SystemParam)]
pub struct SupPrefabs<'w, 's> {
    pub(crate) cmd: Commands<'w, 's>,
}
