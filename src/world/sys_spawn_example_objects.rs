use bevy::prelude::{Commands, StateScoped};
use brg_scene::prelude::InGame;

use crate::prefabs::sup::SupPrefabs;

pub fn spawn_example_objects(mut commands: Commands, mut prefabs: SupPrefabs) {
    let (floor, cube) = prefabs.example_scene();
    commands.spawn((floor, StateScoped(InGame)));
    commands.spawn((cube, StateScoped(InGame)));
}
