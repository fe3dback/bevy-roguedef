use bevy::app::{App, Plugin};
use bevy::prelude::{IntoSystemConfigs, Update};
use brg_editor::prelude::{has_editor_feature, EditorFeature};
use brg_scene::prelude::GameSystemSet;

use super::cmp_projectile::CmpProjectile;
use super::evt::EvtProjectileCollided;
use super::sys_debug_draw_projectile::debug_draw_projectiles;
use super::sys_move_projectiles::move_projectiles;
use super::sys_on_shot::on_shot_spawn_projectile;
use super::sys_play_sound_on_collide::play_sound_on_collide;

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app
            // 
            .register_type::<CmpProjectile>()
            .add_event::<EvtProjectileCollided>()
            .add_systems(Update, on_shot_spawn_projectile.in_set(GameSystemSet::InGame_NOPAUSE_SpawnProjectilesAndEffects))
            .add_systems(Update, debug_draw_projectiles.in_set(GameSystemSet::InGameEditorGizmosDraw).run_if(has_editor_feature(EditorFeature::DrawProjectile)))
            .add_systems(Update, move_projectiles.in_set(GameSystemSet::InGame_NOPAUSE_UpdateMovements))
            .add_systems(Update, play_sound_on_collide.in_set(GameSystemSet::InGame_NOPAUSE_PlaySound))
        //-
        ;
    }
}
