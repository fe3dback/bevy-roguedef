use bevy::prelude::*;
use brg_fundamental::prelude::CmpTransform2D;

use super::cmp_unit_creature::{CmpUnitMovement, CmpUnitMovementInput};

/// Updates the movement of all units based on their movement inputs.
pub fn update_unit_movement(
    mut query: Query<(&CmpUnitMovementInput, &CmpUnitMovement, &mut CmpTransform2D)>,
    time: Res<Time>,
) {
    for (movement_input, movement, mut transform) in query.iter_mut() {
        transform.position += movement_input.direction_vector
            * movement.speed.meters_per_second()
            * time.delta().as_secs_f32();
    }
}
