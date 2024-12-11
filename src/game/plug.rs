use {
    crate::game::{
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
        teams::CmpTeam,
        weapons::{shot, ResPlayerWeapon},
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
        app
            //
            .add_event::<EvtOnDamageCast>()
            //
            .insert_resource(ResMouse::default())
            .insert_resource(ResEnemiesSpawnRules::default())
            .insert_resource(ResPlayerWeapon::default())
            .insert_resource(ResRandomSource {
                rnd: ChaCha8Rng::seed_from_u64(123)
            })
            //
            .register_type::<CmpCollisionDesiredVolume>()
            .register_type::<CmpCollisionCurrentVolume>()
            .register_type::<CmpProjectile>()
            .register_type::<CmpTeam>()
            .register_type::<CmpTimeToLife>()
            .register_type::<CmpHealth>()
            .register_type::<ResPlayerWeapon>()
            .register_type::<ResEnemiesSpawnRules>()
            .register_type::<ResMouse>()
            // systems 
            .add_systems(Update, (update_mouse_pos_resource, remove_expired_ttl_entities))
            .add_systems(Update, (update_collision_volumes, collision_volumes_draw))
            .add_systems(Update, (move_projectiles, draw_projectiles, draw_health_bar, damage_event_listener, death_by_health))
            .add_systems(Update, shot)
            .add_systems(Update, (spawn_enemies, move_enemies_to_castle))

        //-
        ;
    }
}
