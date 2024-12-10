use {
    crate::{
        components::{transform::CmpTransform2D, unit_creature_player::CmpUnitCreaturePlayer},
        game::{
            common::{CmpTimeToLife, ResMouse},
            damage::Damage,
            projectiles::CmpProjectile,
            teams::Team,
        },
    },
    bevy::prelude::{default, ButtonInput, Commands, Entity, MouseButton, Query, Res, With},
};

pub fn shot(
    mut cmd: Commands,
    mouse: Res<ButtonInput<MouseButton>>,
    player_query: Query<(Entity, &CmpTransform2D), With<CmpUnitCreaturePlayer>>,
    mouse_data: Res<ResMouse>,
) {
    if !mouse.just_pressed(MouseButton::Left) {
        return;
    }

    for (player, player_trm) in &player_query {
        cmd.spawn((
            CmpTimeToLife { seconds_left: 2.0 },
            CmpTransform2D {
                position: player_trm.position,
                angle:    player_trm.position.angle_to(mouse_data.world_pos),
            },
            CmpProjectile {
                team: Team::Player,
                caster: Some(player),
                allow_friendly_fire: true,
                damage: Damage {
                    amount: 11.0,
                    dice_faces: 2,
                    dices_amount: 3,
                    ..default()
                },
                ..default()
            },
        ));
    }
}
