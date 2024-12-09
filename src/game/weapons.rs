use {
    crate::{
        components::{transform::CmpTransform2D, unit_creature_player::CmpUnitCreaturePlayer},
        game::{projectiles::CmpProjectile, teams::Team},
    },
    bevy::prelude::{default, ButtonInput, Commands, MouseButton, Query, Res, With},
};

pub fn shot(
    mut cmd: Commands,
    mouse: Res<ButtonInput<MouseButton>>,
    player_query: Query<&CmpTransform2D, With<CmpUnitCreaturePlayer>>,
) {
    if !mouse.just_pressed(MouseButton::Left) {
        return;
    }

    for player_trm in &player_query {
        // todo: angle between origin and mouse

        cmd.spawn((
            CmpTransform2D {
                position: player_trm.position,
                angle:    player_trm.angle,
            },
            CmpProjectile {
                team: Team::Player,
                ..default()
            },
        ));
    }
}
