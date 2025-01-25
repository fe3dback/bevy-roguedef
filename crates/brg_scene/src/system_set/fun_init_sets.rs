use bevy::ecs::schedule::{ScheduleLabel, SystemSetConfigs};
use bevy::prelude::{in_state, App, IntoSystemSetConfigs};
use strum::IntoEnumIterator;

use super::enums::{
    GameSystemSet,
    MAGIC_MATCH_CONDITION_EDITOR,
    MAGIC_MATCH_CONDITION_EDITOR_GIZMOS,
    MAGIC_PREFIX_CONDITION_IN_STATE_IN_GAME_ALWAYS,
    MAGIC_PREFIX_CONDITION_IN_STATE_IN_GAME_NO_PAUSE,
};
use crate::prelude::{has_feature, GameState, InGame, SceneFeature};

pub fn init_system_sets_for(app: &mut App, schedule: impl ScheduleLabel + Clone) {
    let mut prev: Option<GameSystemSet> = None;

    for set in GameSystemSet::iter() {
        let mut stated_set: SystemSetConfigs = set.run_if(|| true);

        // add custom conditions by magic prefix
        if set
            .to_string()
            .starts_with(MAGIC_PREFIX_CONDITION_IN_STATE_IN_GAME_NO_PAUSE)
        {
            stated_set = stated_set.run_if(in_state(GameState::InGame { paused: false }));
        }

        if set
            .to_string()
            .starts_with(MAGIC_PREFIX_CONDITION_IN_STATE_IN_GAME_ALWAYS)
        {
            stated_set = stated_set.run_if(in_state(InGame));
        }

        if set
            .to_string()
            .contains(MAGIC_MATCH_CONDITION_EDITOR_GIZMOS)
        {
            stated_set = stated_set.run_if(has_feature(SceneFeature::EditorGizmos))
        }

        if set.to_string().starts_with(MAGIC_MATCH_CONDITION_EDITOR) {
            stated_set = stated_set.run_if(has_feature(SceneFeature::Editor))
        }

        // add order of execution
        match prev {
            None => app.configure_sets(schedule.clone(), stated_set),
            Some(prev_set) => app.configure_sets(schedule.clone(), stated_set.after(prev_set)),
        };

        prev = Some(set);
    }
}
