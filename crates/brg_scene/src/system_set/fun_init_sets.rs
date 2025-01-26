use bevy::ecs::schedule::{ScheduleLabel, SystemSetConfigs};
use bevy::prelude::{in_state, App, IntoSystemSetConfigs};
use strum::IntoEnumIterator;

use super::enums::{GameSystemSet, MAGIC_ALLOW_ON_LOADING, MAGIC_EDITOR_ONLY, MAGIC_NOT_ON_PAUSE};
use crate::prelude::{has_feature, GameState, Loaded, SceneFeature};

pub fn init_system_sets_for(app: &mut App, schedule: impl ScheduleLabel + Clone) {
    let mut prev: Option<GameSystemSet> = None;

    for set in GameSystemSet::iter() {
        let mut stated_set: SystemSetConfigs = set.run_if(|| true);

        let identity = set.to_string();

        if !identity.contains(MAGIC_ALLOW_ON_LOADING) {
            match identity.contains(MAGIC_NOT_ON_PAUSE) {
                true => {
                    stated_set =
                        stated_set.run_if(in_state(GameState::Loaded { game_paused: false }))
                }
                false => stated_set = stated_set.run_if(in_state(Loaded)),
            };
        }

        if identity.contains(MAGIC_EDITOR_ONLY) {
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
