use {
    crate::game::{buildings::sup::SupBuildingSpawner, common::ResMouse},
    bevy::prelude::{ButtonInput, MouseButton, Res},
};

pub fn spawn_building_on_mouse_click(
    mut manager: SupBuildingSpawner,
    mouse: Res<ButtonInput<MouseButton>>,
    mouse_data: Res<ResMouse>,
) {
    if mouse.just_pressed(MouseButton::Right) {
        manager.spawn_pole(mouse_data.world_pos)
    }
}
