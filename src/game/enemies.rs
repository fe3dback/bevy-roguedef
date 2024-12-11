use {
    crate::{
        components::{
            lib::V2,
            movement::CmpMovement,
            transform::CmpTransform2D,
            unit_creature::CmpUnitCreature,
        },
        game::{
            collisions::CmpCollisionDesiredVolume,
            common::ResRandomSource,
            damage::CmpHealth,
            teams::{CmpTeam, Team},
        },
    },
    bevy::prelude::{
        info,
        AssetServer,
        Circle,
        Commands,
        Component,
        Query,
        Reflect,
        ReflectResource,
        Res,
        ResMut,
        Resource,
        Sprite,
        Time,
        With,
    },
    rand_chacha::rand_core::RngCore,
};

#[derive(Resource, Default, Debug, Reflect)]
#[reflect(Resource)]
pub struct ResEnemiesSpawnRules {
    pub time_to_next_spawn: f32,
}

#[derive(Component, Reflect)]
pub struct CmpEnemyMarkerMoveToCastleAI {}

pub fn spawn_enemies(
    mut cmd: Commands,
    mut rules: ResMut<ResEnemiesSpawnRules>,
    time: Res<Time>,
    mut rand: ResMut<ResRandomSource>,
    asset_server: Res<AssetServer>,
) {
    if rules.time_to_next_spawn > 0.0 {
        rules.time_to_next_spawn -= time.delta().as_secs_f32();
        return;
    }

    // 3000ms to 6000ms
    let rnd_ms = 3000 + (rand.rnd.next_u64() % 3000);
    rules.time_to_next_spawn = (rnd_ms as f32) / 1000.0;

    // 1 to 4
    let rnd_cnt = 1 + (rand.rnd.next_u64() % 3);

    for _ in 0..rnd_cnt {
        let rnd_angle = f32::to_radians((rand.rnd.next_u64() % 360) as f32);
        let rnd_dist = 700.0 + (rand.rnd.next_u64() % 300) as f32;

        let pos_spawn = V2::ZERO.polar_offset(rnd_dist, rnd_angle);

        cmd.spawn((
            CmpTeam {
                team: Team::Enemies,
            },
            CmpEnemyMarkerMoveToCastleAI {},
            CmpTransform2D {
                position: pos_spawn,
                angle:    pos_spawn.angle_to(V2::ZERO),
            },
            CmpHealth {
                health:     80.0,
                max_health: 80.0,
            },
            CmpCollisionDesiredVolume::Circle(Circle::new(24.0)),
            CmpUnitCreature::default(),
            Sprite::from_image(asset_server.load("sprites/creatures/ghost.png")),
        ));
    }
}

pub fn move_enemies_to_castle(
    mut enemies_q: Query<(&mut CmpTransform2D, &CmpMovement), With<CmpEnemyMarkerMoveToCastleAI>>,
    time: Res<Time>,
) {
    for (mut transform, movement) in &mut enemies_q {
        let cur_pos = transform.position;
        let next_pos = cur_pos.polar_offset(
            movement.speed * time.delta().as_secs_f32(),
            cur_pos.angle_to(V2::ZERO),
        );

        transform.position = next_pos;
    }
}
