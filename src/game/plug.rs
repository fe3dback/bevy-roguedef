use crate::game::collisions::collision_volumes_draw;
use crate::game::weapons::shot;
use {
    crate::game::{
        collisions::{
            update_collision_volumes,
            CmpCollisionCurrentVolume,
            CmpCollisionDesiredVolume,
        },
        projectiles::{draw_projectiles, move_projectiles, CmpProjectile},
        teams::CmpTeam,
    },
    bevy::{
        app::{App, Plugin},
        prelude::Update,
    },
};

pub struct Plug {}

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app
            //
            .register_type::<CmpCollisionDesiredVolume>()
            .register_type::<CmpCollisionCurrentVolume>()
            .register_type::<CmpProjectile>()
            .register_type::<CmpTeam>()
            // systems
            .add_systems(Update, (update_collision_volumes, collision_volumes_draw))
            .add_systems(Update, (move_projectiles, draw_projectiles))
            .add_systems(Update, shot)

        //-
        ;
    }
}
