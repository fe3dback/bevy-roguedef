use bevy::app::{App, Plugin};
use bevy::prelude::Update;
use rand_chacha::rand_core::SeedableRng;
use rand_chacha::ChaCha8Rng;

use crate::game::ai::ai_auto_attack_nearest_enemies;
use crate::game::buildings;
use crate::game::collisions::{
    update_collision_volumes,
    CmpCollisionCurrentVolume,
    CmpCollisionDesiredVolume,
};
use crate::game::common::{
    remove_expired_ttl_entities,
    update_mouse_pos_resource,
    CmpTimeToLife,
    ResMouse,
    ResRandomSource,
};
use crate::game::damage::{
    damage_event_listener,
    death_by_health,
    draw_health_bar,
    CmpHealth,
    EvtOnDamageCast,
};
use crate::game::enemies::{move_enemies_to_castle, spawn_enemies, ResEnemiesSpawnRules};
use crate::game::projectiles::{move_projectiles, CmpProjectile};
use crate::game::sound::ResRandomSoundSource;
use crate::game::teams::CmpTeam;
use crate::game::weapons::{auto_reset_weapon_trigger, player_trigger_shot, shooting, CmpWeapon};

pub struct Plug {}

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        let seed = 123;

        app
            //
            .add_plugins(buildings::plug::Plug {})
            //
            .add_event::<EvtOnDamageCast>()
            //
            .insert_resource(ResMouse::default())
            .insert_resource(ResEnemiesSpawnRules::default())
            .insert_resource(ResRandomSource {
                rnd: ChaCha8Rng::seed_from_u64(seed)
            })
            .insert_resource(ResRandomSoundSource {
                rnd: ChaCha8Rng::seed_from_u64(seed)
            })
            //
            .register_type::<CmpCollisionDesiredVolume>()
            .register_type::<CmpCollisionCurrentVolume>()
            .register_type::<CmpProjectile>()
            .register_type::<CmpTeam>()
            .register_type::<CmpTimeToLife>()
            .register_type::<CmpHealth>()
            .register_type::<CmpWeapon>()
            .register_type::<ResEnemiesSpawnRules>()
            .register_type::<ResMouse>()
            // systems 
            .add_systems(Update, auto_reset_weapon_trigger)
            .add_systems(Update, ai_auto_attack_nearest_enemies)
            .add_systems(Update, (update_mouse_pos_resource, remove_expired_ttl_entities))
            .add_systems(Update, (update_collision_volumes))
            .add_systems(Update, (move_projectiles, draw_health_bar, damage_event_listener, death_by_health))
            .add_systems(Update, (player_trigger_shot, shooting))
            .add_systems(Update, (spawn_enemies, move_enemies_to_castle))

        //-
        ;
    }
}
