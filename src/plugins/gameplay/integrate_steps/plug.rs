use bevy::app::{App, Plugin, Update};
use bevy::prelude::Startup;

use crate::plugins::gameplay::integrate_steps::evt::EvtOnIntegration;
use crate::plugins::gameplay::integrate_steps::sys;
use crate::plugins::gameplay::integrate_steps::sys::ResLocalIntegrationTimings;

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app
            //
            .insert_resource(ResLocalIntegrationTimings::default())
            .add_event::<EvtOnIntegration>()
            .add_systems(Startup, sys::init)
            // .add_systems(
            //     OnExit(ELoadingState::Scene),
            //     sys::clear.in_set(ESystemCreateSet::SpawnSystemStaff),
            // )
            .add_systems(Update, sys::update)
            .add_systems(Update, sys::editor_update);
    }
}
