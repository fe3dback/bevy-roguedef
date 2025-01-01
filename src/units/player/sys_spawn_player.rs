use bevy::prelude::{Commands, StateScoped};
use brg_scene::prelude::InGame;

use crate::prefabs::sup_prefabs::SupPrefabs;

pub fn spawn_player(mut commands: Commands, mut prefabs: SupPrefabs) {
    commands.spawn((prefabs.player(), StateScoped(InGame)));
}
