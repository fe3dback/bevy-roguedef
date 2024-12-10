use {
    crate::{
        components::{
            lib::V2,
            movement::{CmpMarkerMovementRestrictInPlayableArea, CmpMovement},
            transform::CmpTransform2D,
        },
        consts::PLAYABLE_AREA_SIZE,
    },
    bevy::prelude::{Query, Res, Time, With},
};

pub fn apply_movement(mut query: Query<(&mut CmpTransform2D, &CmpMovement)>, time: Res<Time>) {
    for (mut transform, movement) in &mut query {
        if movement.ctl_input == V2::ZERO {
            continue;
        }

        transform.position += movement.ctl_input * movement.speed * time.delta().as_secs_f32();
    }
}

pub fn restrict_movement_in_playable_area(
    mut query: Query<(&mut CmpTransform2D), With<CmpMarkerMovementRestrictInPlayableArea>>,
) {
    for mut transform in &mut query {
        let distance = transform.position.distance(V2::ZERO);
        if distance < PLAYABLE_AREA_SIZE {
            continue;
        }

        let polar =
            V2::ZERO.polar_offset(PLAYABLE_AREA_SIZE, V2::ZERO.angle_to(transform.position));
        transform.position = polar;
    }
}
