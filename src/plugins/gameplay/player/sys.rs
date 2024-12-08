use crate::prefabs::sup::SupPrefabs;
use bevy::prelude::Commands;

pub fn spawn_player(mut cmd: Commands, mut prefabs: SupPrefabs) {
    cmd.spawn(
        prefabs.player()
    );
}