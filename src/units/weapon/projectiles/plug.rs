use bevy::app::{App, Plugin};
use bevy::prelude::{IntoSystemConfigs, Update};
use brg_editor::prelude::{has_editor_feature, EditorFeature};
use brg_scene::prelude::GameSystemSet;

use crate::units::weapon::projectiles::cmp_projectile::CmpProjectile;
use crate::units::weapon::projectiles::sys_debug_draw_projectile::debug_draw_projectiles;
use crate::units::weapon::projectiles::sys_on_shot::on_shot_spawn_projectile;

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app
        //
            .register_type::<CmpProjectile>()
            .add_systems(Update, on_shot_spawn_projectile.in_set(GameSystemSet::InGameSpawnProjectilesAndEffects))
            .add_systems(Update, debug_draw_projectiles.in_set(GameSystemSet::InGameEditorGizmosDraw).run_if(has_editor_feature(EditorFeature::DrawProjectile)))
        //-
        ;
    }
}
