use std::time::Duration;

use bevy::prelude::{
    default,
    info,
    AlphaMode,
    AssetServer,
    Circle,
    Commands,
    Component,
    Mut,
    Query,
    Reflect,
    ReflectResource,
    Res,
    ResMut,
    Resource,
    Sprite,
    Time,
    With,
    Without,
    World,
};
use bevy::window::PrimaryWindow;
use bevy_inspector_egui::bevy_egui::EguiContext;
use bevy_inspector_egui::egui;
use bevy_sprite3d::{Sprite3dBuilder, Sprite3dParams};
use brg_core::prelude::{CmpTransform2D, V2};
use rand_chacha::rand_core::RngCore;

use crate::components::ai::{CmpEnemyMarkerAttackWhenNear, CmpEnemyMarkerMoveToCastleAI};
use crate::components::movement::CmpMovement;
use crate::components::unit::EUnitType;
use crate::components::unit_creature::CmpUnitCreature;
use crate::components::unit_creature_player::CmpUnitCreaturePlayer;
use crate::game::collisions::CmpCollisionDesiredVolume;
use crate::game::common::ResRandomSource;
use crate::game::damage::{CmpHealth, Damage, DamageKind};
use crate::game::teams::{CmpTeam, Team};
use crate::game::weapons::{CmpWeapon, Weapon};
use crate::plugins::assets::asset::GameAssets;
use crate::plugins::gameplay::integrate_steps::enums::EventType;

pub fn spawn_enemies(
    mut cmd: Commands,
    mut rules: ResMut<ResEnemiesSpawnRules>,
    mut rand: ResMut<ResRandomSource>,
    mut sprite_params: Sprite3dParams,
    assets: Res<GameAssets>,
) {
    if !rules.spawn_clicked {
        return;
    }

    rules.spawn_clicked = false;

    // 1 to 4
    let rnd_cnt = 1 + (rand.rnd.next_u64() % (rules.dice_sides * rules.dice_count) as u64);

    for _ in 0..rnd_cnt {
        let rnd_angle = f32::to_radians((rand.rnd.next_u64() % 360) as f32);
        let rnd_dist = 30.0 + (rand.rnd.next_u64() % 10) as f32;

        let pos_spawn = V2::ZERO.polar_offset(rnd_dist, rnd_angle);

        cmd.spawn((
            CmpTeam {
                team: Team::Enemies,
            },
            EUnitType::Creature,
            CmpEnemyMarkerMoveToCastleAI {},
            CmpEnemyMarkerAttackWhenNear {},
            CmpTransform2D {
                position: pos_spawn,
                angle: pos_spawn.angle_to(V2::ZERO),
                ..default()
            },
            CmpHealth {
                health:     80.0,
                max_health: 80.0,
            },
            CmpWeapon {
                current: Weapon {
                    name:                 String::from("claws"),
                    shooting_reload_time: Duration::from_millis(500),
                    magazine_reload_time: Duration::from_millis(700),
                    damage:               Damage {
                        amount: 1.0,
                        kind: DamageKind::Melee,
                        ..default()
                    },
                    ammo_in_magazine:     2,
                },
                ..default()
            },
            CmpCollisionDesiredVolume::Circle(Circle::new(1.0)),
            CmpUnitCreature::default(),
            Sprite3dBuilder {
                image: assets
                    .sprites
                    .get("sprites/creatures/ghost.png")
                    .unwrap()
                    .clone_weak(),
                pixels_per_metre: 24.0,
                alpha_mode: AlphaMode::Blend,
                unlit: true,
                double_sided: true,
                ..default()
            }
            .bundle(&mut sprite_params),
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
