use {
    crate::components::{lib::V2, movement::CmpMovement, transform::CmpTransform2D},
    bevy::prelude::{Query, Res, Time},
};

pub fn apply_movement(mut query: Query<(&mut CmpTransform2D, &CmpMovement)>, time: Res<Time>) {
    for (mut transform, movement) in &mut query {
        if movement.ctl_input == V2::ZERO {
            continue;
        }

        transform.position += movement.ctl_input * movement.speed * time.delta().as_secs_f32();
    }
}
