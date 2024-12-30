use bevy::prelude::*;
use brg_core::prelude::V2;

use crate::units::cmp_unit_creature::CmpUnitMovementInput;
use crate::units::player::cmp_marker_player::CmpMarkerPlayer;

pub fn update_movement_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut CmpUnitMovementInput, With<CmpMarkerPlayer>>,
) {
    for mut mov_input in query.iter_mut() {
        let mut direction_vector = V2::ZERO;

        if keyboard_input.pressed(KeyCode::KeyW) {
            direction_vector.y = -1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            direction_vector.y = 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            direction_vector.x = -1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            direction_vector.x = 1.0;
        }

        mov_input.direction_vector = direction_vector.normalize();
    }
}
