use bevy::app::{App, Plugin};
use bevy::prelude::{IntoSystemConfigs, Update};
use brg_scene::prelude::{has_feature, GameSystemSet, SceneFeature};

use crate::units::mobs::res_spawn::ResMobsSpawnRules;
use crate::units::mobs::sys_spawn::{editor_enemies_window_update, spawn_mobs};

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app
            //
            .register_type::<ResMobsSpawnRules>()
            .insert_resource(ResMobsSpawnRules::default())
            .add_systems(Update, editor_enemies_window_update.in_set(GameSystemSet::EDITOR_ONLY__DrawEditorEguiPanels).run_if(has_feature(SceneFeature::Units)))
            .add_systems(Update, spawn_mobs.in_set(GameSystemSet::NOT_ON_PAUSE__SpawnMobs).run_if(has_feature(SceneFeature::Units)))
        //-
        ;
    }
}
