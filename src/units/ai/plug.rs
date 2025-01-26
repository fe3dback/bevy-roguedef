use bevy::app::{App, Plugin};
use bevy::prelude::{IntoSystemConfigs, Update};
use brg_scene::prelude::GameSystemSet;

use super::cmp_ai::CmpAiBehaviorSimple;
use super::sys_ai_behavior_simple::ai_simple_move_to_enemy;

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app
            //
            .register_type::<CmpAiBehaviorSimple>()
            .add_systems(Update, ai_simple_move_to_enemy.in_set(GameSystemSet::NOT_ON_PAUSE__ProcessInput))
        //-
        ;
    }
}
