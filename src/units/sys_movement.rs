use bevy::prelude::*;
use brg_fundamental::prelude::CmpTransform2D;

use super::cmp_unit_creature::CmpUnitMovementInput;

/// Updates the movement of all units based on their movement inputs.
pub fn update_unit_movement(
    mut query: Query<(&CmpUnitMovementInput, &mut CmpTransform2D)>,
    time: Res<Time>,
) {
    for (movement_input, mut transform) in query.iter_mut() {
        transform.position += movement_input.direction_vector
            * movement_input.speed.meters_per_second()
            * time.delta().as_secs_f32();
    }
}
