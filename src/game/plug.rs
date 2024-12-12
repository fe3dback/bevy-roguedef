use {
    crate::game::{
        buildings,
        collisions::{
            collision_volumes_draw,
            update_collision_volumes,
            CmpCollisionCurrentVolume,
            CmpCollisionDesiredVolume,
        },
        common::{
            remove_expired_ttl_entities,
            update_mouse_pos_resource,
            CmpTimeToLife,
            ResMouse,
            ResRandomSource,
        },
        damage::{
            damage_event_listener,
            death_by_health,
            draw_health_bar,
            CmpHealth,
            EvtOnDamageCast,
        },
        enemies::{move_enemies_to_castle, spawn_enemies, ResEnemiesSpawnRules},
        projectiles::{draw_projectiles, move_projectiles, CmpProjectile},
        sound::ResRandomSoundSource,
        teams::CmpTeam,
        weapons::{auto_reset_weapon_trigger, player_trigger_shot, shooting, CmpWeapon},
    },
    bevy::{
        app::{App, Plugin},
        prelude::Update,
    },
    rand_chacha::{rand_core::SeedableRng, ChaCha8Rng},
};

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
            .add_systems(Update, (update_mouse_pos_resource, remove_expired_ttl_entities))
            .add_systems(Update, (update_collision_volumes, collision_volumes_draw))
            .add_systems(Update, (move_projectiles, draw_projectiles, draw_health_bar, damage_event_listener, death_by_health))
            .add_systems(Update, (player_trigger_shot, shooting))
            .add_systems(Update, (spawn_enemies, move_enemies_to_castle))

        //-
        ;
    }
}
