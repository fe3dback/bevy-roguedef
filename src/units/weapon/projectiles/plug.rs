use bevy::app::{App, Plugin};
use bevy::prelude::{IntoSystemConfigs, Update};
use brg_scene::prelude::{has_editor_feature, EditorFeature, GameSystemSet};

use super::cmp_projectile::CmpProjectile;
use super::evt::EvtProjectileCollided;
use super::sys_debug_draw_projectile::debug_draw_projectiles;
use super::sys_move_projectiles::move_projectiles;
use super::sys_on_collide::on_collide;
use super::sys_on_shot::on_shot_spawn_projectile;

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app
            // 
            .register_type::<CmpProjectile>()
            .add_event::<EvtProjectileCollided>()
            .add_systems(Update, on_shot_spawn_projectile.in_set(GameSystemSet::NOT_ON_PAUSE__SpawnProjectilesAndEffects))
            .add_systems(Update, debug_draw_projectiles.in_set(GameSystemSet::GizmosDraw).run_if(has_editor_feature(EditorFeature::VolumesProjectile)))
            .add_systems(Update, move_projectiles.in_set(GameSystemSet::NOT_ON_PAUSE__UpdateMovements))
            .add_systems(Update, on_collide.in_set(GameSystemSet::NOT_ON_PAUSE__CastSpells))
        //-
        ;
    }
}
