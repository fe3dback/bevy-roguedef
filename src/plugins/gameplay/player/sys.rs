use {
    crate::{
        components::{lib::V2, movement::CmpMovement, unit_creature_player::CmpUnitCreaturePlayer},
        prefabs::sup::SupPrefabs,
    },
    bevy::{
        input::ButtonInput,
        prelude::{Commands, KeyCode, Query, Res, With},
    },
};

pub fn spawn_player(mut cmd: Commands, mut prefabs: SupPrefabs) {
    cmd.spawn(prefabs.player());
}

pub fn wasd_movement(
    mut query: Query<&mut CmpMovement, With<CmpUnitCreaturePlayer>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    for mut ctl in &mut query {
        let mut movement_vec = V2::ZERO;

        if keyboard.pressed(KeyCode::KeyW) {
            movement_vec.y = -1.0;
        }

        if keyboard.pressed(KeyCode::KeyS) {
            movement_vec.y = 1.0;
        }

        if keyboard.pressed(KeyCode::KeyA) {
            movement_vec.x = -1.0;
        }

        if keyboard.pressed(KeyCode::KeyD) {
            movement_vec.x = 1.0;
        }

        ctl.ctl_input = movement_vec;
    }
}
