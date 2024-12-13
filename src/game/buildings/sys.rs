use bevy::prelude::{ButtonInput, MouseButton, Res};

use crate::game::buildings::sup::SupBuildingSpawner;
use crate::game::common::ResMouse;

pub fn spawn_building_on_mouse_click(
    mut manager: SupBuildingSpawner,
    mouse: Res<ButtonInput<MouseButton>>,
    mouse_data: Res<ResMouse>,
) {
    if mouse.just_pressed(MouseButton::Right) {
        manager.spawn_tower(mouse_data.world_pos)
    }
}
