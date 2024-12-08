use crate::components::lib::V2;
use crate::components::movement::CmpMovement;
use crate::components::transform::CmpTransform2D;
use bevy::prelude::{Query, Res, Time};

pub fn apply_movement(mut query: Query<(&mut CmpTransform2D, &CmpMovement)>, time: Res<Time>) {
    for (mut transform, movement) in &mut query {
        if movement.ctl_input == V2::ZERO {
            continue;
        }

        transform.position += movement.ctl_input * movement.speed * time.delta().as_secs_f32();
    }
}